//! FFI interface layer.
//!
//! Plain Rust functions that mirror every HTTP route.  No `flutter_rust_bridge`
//! dependency here — that lives in the `netflow-ffi` crate which re-exports
//! these and applies `#[frb]` annotations so the codegen tool can find them.
//!
//! # Initialisation
//!
//! Call `init(db_path, user_id)` once before anything else.
//! A single `tokio` Runtime and `AppState` are stored in a process-global
//! `OnceLock` — safe because Flutter's Dart isolate calls into Rust on one
//! thread at a time, and the state itself is `Arc`-backed and `Clone`.

use std::sync::{Arc, OnceLock};
use tokio::runtime::Runtime;
use uuid::Uuid;
use rust_decimal::Decimal;
use std::str::FromStr;

use crate::{
    application::{
        error::LedgerError,
        ports::{
            LedgerEntryRepository, LedgerUnitOfWork,
            StatementRepository, UserFinancesRepository,
        },
        use_cases::{
            accrue_interest, add_credit_card, add_digital_wallet,
            add_holding, add_physical_wallet, charge_credit_card,
            close_statement_with_record, create_user_finances,
            deposit_funds, get_net_worth, grant_temporary_limit,
            list_accounts, make_payment, open_cash_account,
            open_investment_account, open_loan_account,
            remove_account, remove_holding, revoke_temporary_limit,
            transfer_funds, update_account_info, update_holding_price,
            withdraw_funds,
        },
    },
    domain::{account_id::AccountId, currency::Currency},
    infrastructure::{
        open_db,
        SqliteLedgerEntryRepository, SqliteLedgerUnitOfWork,
        SqliteStatementRepository, SqliteUserFinancesRepository,
    },
    interface::resolve_user,
};
use shared::domain::UserId;

// ── Process-global state ──────────────────────────────────────────────────────

struct State {
    rt:             Runtime,
    repo:           Arc<dyn UserFinancesRepository>,
    entry_repo:     Arc<dyn LedgerEntryRepository>,
    statement_repo: Arc<dyn StatementRepository>,
    uow:            Arc<dyn LedgerUnitOfWork>,
    user_id:        UserId,
}

// Arc internals are Send+Sync; OnceLock write happens exactly once.
unsafe impl Send for State {}
unsafe impl Sync for State {}

static STATE: OnceLock<State> = OnceLock::new();

fn s() -> &'static State {
    STATE.get().expect("ledger FFI: call init() before any other function")
}

// ── Public DTOs ───────────────────────────────────────────────────────────────
// Primitive-only structs so netflow-ffi can expose them to flutter_rust_bridge
// without the ledger crate depending on frb.

pub struct AccountSummary {
    pub account_id:   String,
    pub account_name: String,
    pub account_type: String,
    pub currency:     String,
    pub balance:      String,
    pub is_debt:      bool,
    pub is_overdue:   bool,
}

pub struct NetWorthResult {
    pub currency:     String,
    pub total_assets: String,
    pub total_debts:  String,
    pub net_worth:    String,
    pub is_deficit:   bool,
}

pub struct LedgerEntry {
    pub id:          String,
    pub account_id:  String,
    pub entry_type:  String,
    pub amount:      String,
    pub currency:    String,
    pub occurred_at: String,
    pub label:       Option<String>,
    pub description: Option<String>,
}

pub struct Statement {
    pub id:                String,
    pub account_id:        String,
    pub cycle_start:       String,
    pub cycle_end:         String,
    pub statement_balance: String,
    pub minimum_payment:   Option<String>,
    pub total_charged:     String,
    pub total_paid:        String,
    pub is_settled:        bool,
}

// Detail DTOs mirroring the HTTP account_detail handlers

pub struct CashAccountDetail {
    pub account_id:     String,
    pub account_name:   String,
    pub currency:       String,
    pub balance:        String,
    pub bank:           String,
    pub account_number: String,
}

pub struct PhysicalWalletDetail {
    pub account_id:   String,
    pub account_name: String,
    pub currency:     String,
    pub balance:      String,
}

pub struct DigitalWalletDetail {
    pub account_id:          String,
    pub account_name:        String,
    pub currency:            String,
    pub balance:             String,
    pub provider:            String,
    pub provider_account_id: String,
}

pub struct HoldingDetail {
    pub ticker:          String,
    pub investment_type: String,
    pub quantity:        String,
    pub unit_price:      String,
    pub market_value:    String,
    pub currency:        String,
}

pub struct InvestmentAccountDetail {
    pub account_id:     String,
    pub account_name:   String,
    pub currency:       String,
    pub cash_balance:   String,
    pub holdings_value: String,
    pub total_value:    String,
    pub bank:           String,
    pub account_number: String,
    pub holdings:       Vec<HoldingDetail>,
}

pub struct LoanAccountDetail {
    pub account_id:      String,
    pub account_name:    String,
    pub currency:        String,
    pub bank:            String,
    pub account_number:  Option<String>,
    pub creditor:        String,
    pub principal:       String,
    pub outstanding:     String,
    pub amount_paid:     String,
    pub percent_paid:    String,
    pub interest_rate:   Option<String>,
    pub due_day:         Option<u8>,
    pub maturity_date:   Option<String>,
    pub minimum_payment: Option<String>,
    pub is_overdue:      bool,
    pub is_settled:      bool,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn cur(s: &str) -> Result<Currency, String> {
    s.parse::<Currency>().map_err(|e| e.to_string())
}

fn dec(s: &str) -> Result<Decimal, String> {
    Decimal::from_str(s).map_err(|e| e.to_string())
}

fn aid(s: &str) -> Result<AccountId, String> {
    let u = Uuid::parse_str(s).map_err(|e| e.to_string())?;
    AccountId::restore(u).map_err(|e| e.to_string())
}

fn uid(s: &str) -> Result<Uuid, String> {
    Uuid::parse_str(s).map_err(|e| e.to_string())
}

fn e(err: LedgerError) -> String { err.to_string() }

// ── Initialisation ────────────────────────────────────────────────────────────

/// Initialise the FFI layer. Must be called once before anything else.
/// Repeated calls are silently ignored (idempotent).
///
/// `db_path`    — absolute path to the SQLite file; created if absent.
/// `user_id`    — optional UUID string; defaults to the single-user sentinel.
pub fn init(db_path: String, user_id: Option<String>) -> Result<(), String> {
    if STATE.get().is_some() { return Ok(()); }

    // Ensure parent directory exists.
    if let Some(p) = std::path::Path::new(&db_path).parent() {
        if !p.as_os_str().is_empty() && !p.exists() {
            std::fs::create_dir_all(p)
                .map_err(|err| format!("cannot create db dir: {err}"))?;
        }
    }

    let rt   = Runtime::new().map_err(|err| err.to_string())?;
    let pool = rt.block_on(open_db(&db_path)).map_err(|err| err.to_string())?;

    let user_uuid = user_id.as_deref()
        .map(|s| Uuid::parse_str(s).map_err(|err| err.to_string()))
        .transpose()?;

    STATE.set(State {
        user_id:        resolve_user(user_uuid),
        repo:           Arc::new(SqliteUserFinancesRepository::new(pool.clone())),
        entry_repo:     Arc::new(SqliteLedgerEntryRepository::new(pool.clone())),
        statement_repo: Arc::new(SqliteStatementRepository::new(pool.clone())),
        uow:            Arc::new(SqliteLedgerUnitOfWork::new(pool)),
        rt,
    }).map_err(|_| "init() already called".to_string())
}

// ── Lifecycle ─────────────────────────────────────────────────────────────────

pub fn init_finances() -> Result<(), String> {
    let s = s();
    s.rt.block_on(create_user_finances::execute(
        s.uow.as_ref(),
        create_user_finances::CreateUserFinancesCommand { owner_id: s.user_id },
    )).map_err(e)
}

// ── Accounts — queries ────────────────────────────────────────────────────────

pub fn list_accounts() -> Result<Vec<AccountSummary>, String> {
    let s = s();
    s.rt.block_on(list_accounts::execute(
        s.repo.as_ref(),
        list_accounts::ListAccountsQuery { owner_id: s.user_id },
    ))
    .map(|v| v.into_iter().map(|a| AccountSummary {
        account_id:   a.account_id,
        account_name: a.account_name,
        account_type: a.account_type.to_string(),
        currency:     a.currency,
        balance:      a.balance,
        is_debt:      a.is_debt,
        is_overdue:   a.is_overdue,
    }).collect())
    .map_err(e)
}

pub fn net_worth(currency: Option<String>) -> Result<Vec<NetWorthResult>, String> {
    let s   = s();
    let cur = currency.as_deref().map(|c| c.parse::<Currency>().map_err(|err| err.to_string())).transpose()?;
    s.rt.block_on(get_net_worth::execute(
        s.repo.as_ref(),
        get_net_worth::GetNetWorthQuery { owner_id: s.user_id, currency: cur },
    ))
    .map(|v| v.into_iter().map(|r| NetWorthResult {
        currency:     r.currency,
        total_assets: r.total_assets,
        total_debts:  r.total_debts,
        net_worth:    r.net_worth,
        is_deficit:   r.is_deficit,
    }).collect())
    .map_err(e)
}

// ── Account detail queries ────────────────────────────────────────────────────

pub fn cash_detail(account_id: String) -> Result<CashAccountDetail, String> {
    let s       = s();
    let acct_id = aid(&account_id)?;
    let finances = s.rt.block_on(s.repo.load(s.user_id)).map_err(e)?;
    let a = finances.find_account(acct_id)
        .ok_or_else(|| LedgerError::AccountNotFound(acct_id.uuid()).to_string())?
        .as_cash()
        .ok_or_else(|| LedgerError::WrongAccountType(acct_id.uuid()).to_string())?;
    Ok(CashAccountDetail {
        account_id:     a.account_id().to_string(),
        account_name:   a.account_name().value().to_string(),
        currency:       format!("{:?}", a.currency()),
        balance:        a.balance().amount().to_string(),
        bank:           a.bank().value().to_string(),
        account_number: a.account_number().value().to_string(),
    })
}

pub fn wallet_detail(account_id: String) -> Result<PhysicalWalletDetail, String> {
    let s        = s();
    let acct_id  = aid(&account_id)?;
    let finances = s.rt.block_on(s.repo.load(s.user_id)).map_err(e)?;
    let a = finances.find_account(acct_id)
        .ok_or_else(|| LedgerError::AccountNotFound(acct_id.uuid()).to_string())?
        .as_physical_wallet()
        .ok_or_else(|| LedgerError::WrongAccountType(acct_id.uuid()).to_string())?;
    Ok(PhysicalWalletDetail {
        account_id:   a.account_id().to_string(),
        account_name: a.account_name().value().to_string(),
        currency:     format!("{:?}", a.currency()),
        balance:      a.balance().amount().to_string(),
    })
}

pub fn digital_wallet_detail(account_id: String) -> Result<DigitalWalletDetail, String> {
    let s        = s();
    let acct_id  = aid(&account_id)?;
    let finances = s.rt.block_on(s.repo.load(s.user_id)).map_err(e)?;
    let a = finances.find_account(acct_id)
        .ok_or_else(|| LedgerError::AccountNotFound(acct_id.uuid()).to_string())?
        .as_digital_wallet()
        .ok_or_else(|| LedgerError::WrongAccountType(acct_id.uuid()).to_string())?;
    Ok(DigitalWalletDetail {
        account_id:          a.account_id().to_string(),
        account_name:        a.account_name().value().to_string(),
        currency:            format!("{:?}", a.currency()),
        balance:             a.balance().amount().to_string(),
        provider:            a.provider().to_string(),
        provider_account_id: a.provider_account_id().value().to_string(),
    })
}

pub fn investment_detail(account_id: String) -> Result<InvestmentAccountDetail, String> {
    let s        = s();
    let acct_id  = aid(&account_id)?;
    let finances = s.rt.block_on(s.repo.load(s.user_id)).map_err(e)?;
    let a = finances.find_account(acct_id)
        .ok_or_else(|| LedgerError::AccountNotFound(acct_id.uuid()).to_string())?
        .as_investment()
        .ok_or_else(|| LedgerError::WrongAccountType(acct_id.uuid()).to_string())?;
    let holdings_value = a.holdings_value().map(|m| m.amount().to_string()).unwrap_or_else(|_| "0".into());
    let total_value    = a.total_value().map(|m| m.amount().to_string()).unwrap_or_else(|_| a.cash_balance().amount().to_string());
    let holdings = a.holdings().iter().map(|h| HoldingDetail {
        ticker:          h.ticker().value().to_string(),
        investment_type: format!("{:?}", h.investment_type()),
        quantity:        h.quantity().to_string(),
        unit_price:      h.unit_price().amount().to_string(),
        market_value:    h.market_value().map(|m| m.amount().to_string()).unwrap_or_else(|_| "0".into()),
        currency:        format!("{:?}", h.unit_price().currency()),
    }).collect();
    Ok(InvestmentAccountDetail {
        account_id:     a.account_id().to_string(),
        account_name:   a.account_name().value().to_string(),
        currency:       format!("{:?}", a.currency()),
        cash_balance:   a.cash_balance().amount().to_string(),
        holdings_value,
        total_value,
        bank:           a.bank().value().to_string(),
        account_number: a.account_number().value().to_string(),
        holdings,
    })
}

pub fn loan_detail(account_id: String) -> Result<LoanAccountDetail, String> {
    let s        = s();
    let acct_id  = aid(&account_id)?;
    let finances = s.rt.block_on(s.repo.load(s.user_id)).map_err(e)?;
    let a = finances.find_account(acct_id)
        .ok_or_else(|| LedgerError::AccountNotFound(acct_id.uuid()).to_string())?
        .as_loan()
        .ok_or_else(|| LedgerError::WrongAccountType(acct_id.uuid()).to_string())?;
    let loan        = a.loan();
    let principal   = loan.principal().amount();
    let outstanding = loan.outstanding().amount();
    let paid        = principal - outstanding;
    let pct = if principal.is_zero() { "0.00".into() }
              else { format!("{:.2}", paid / principal * Decimal::ONE_HUNDRED) };
    Ok(LoanAccountDetail {
        account_id:      a.account_id().to_string(),
        account_name:    a.account_name().value().to_string(),
        currency:        format!("{:?}", a.currency()),
        bank:            a.bank().value().to_string(),
        account_number:  a.account_number().map(|n| n.value().to_string()),
        creditor:        loan.creditor().to_string(),
        principal:       principal.to_string(),
        outstanding:     outstanding.to_string(),
        amount_paid:     paid.to_string(),
        percent_paid:    pct,
        interest_rate:   loan.interest_rate().map(|r| r.to_string()),
        due_day:         loan.due_date().map(|d| d.value()),
        maturity_date:   loan.maturity_date().map(|d| d.to_string()),
        minimum_payment: loan.minimum_payment().map(|l| l.amount().to_string()),
        is_overdue:      loan.is_overdue(),
        is_settled:      loan.is_settled(),
    })
}

// ── Accounts — mutations ──────────────────────────────────────────────────────

pub fn remove_account(account_id: String) -> Result<(), String> {
    let s = s();
    s.rt.block_on(remove_account::execute(s.uow.as_ref(), remove_account::RemoveAccountCommand {
        owner_id:   s.user_id,
        account_id: aid(&account_id)?,
    })).map_err(e)
}

pub fn update_account_info(account_id: String, name: Option<String>, bank: Option<String>, account_number: Option<String>) -> Result<(), String> {
    let s = s();
    let u = uid(&account_id)?;
    s.rt.block_on(update_account_info::execute(s.uow.as_ref(), update_account_info::UpdateAccountInfoCommand {
        owner_id: s.user_id, account_id: u, name, bank, account_number,
    })).map_err(e)
}

// ── Open / add accounts ───────────────────────────────────────────────────────

pub fn open_cash_account(name: String, account_number: String, bank: String, currency: String, initial_balance: String) -> Result<(), String> {
    let s = s();
    s.rt.block_on(open_cash_account::execute(s.uow.as_ref(), open_cash_account::OpenCashAccountCommand {
        owner_id: s.user_id, name, account_number, bank,
        currency:        cur(&currency)?,
        initial_balance: dec(&initial_balance)?,
    })).map_err(e)
}

pub fn add_physical_wallet(name: String, currency: String, initial_balance: String) -> Result<(), String> {
    let s = s();
    s.rt.block_on(add_physical_wallet::execute(s.uow.as_ref(), add_physical_wallet::AddPhysicalWalletCommand {
        owner_id: s.user_id, name,
        currency:        cur(&currency)?,
        initial_balance: dec(&initial_balance)?,
    })).map_err(e)
}

pub fn add_digital_wallet(name: String, provider: String, provider_account_id: String, currency: String, initial_balance: String) -> Result<(), String> {
    let s = s();
    s.rt.block_on(add_digital_wallet::execute(s.uow.as_ref(), add_digital_wallet::AddDigitalWalletCommand {
        owner_id: s.user_id, name, provider, provider_account_id,
        currency:        cur(&currency)?,
        initial_balance: dec(&initial_balance)?,
    })).map_err(e)
}

pub fn open_investment_account(name: String, account_number: String, bank: String, currency: String, cash_balance: String) -> Result<(), String> {
    let s = s();
    s.rt.block_on(open_investment_account::execute(s.uow.as_ref(), open_investment_account::OpenInvestmentAccountCommand {
        owner_id: s.user_id, name, account_number, bank,
        currency:     cur(&currency)?,
        cash_balance: dec(&cash_balance)?,
    })).map_err(e)
}

pub fn add_credit_card(
    name: String, last_four: String, network: String,
    expiry_month: u8, expiry_year: u16,
    credit_limit: String, currency: String,
    outstanding: Option<String>, cash_advance_limit: Option<String>,
    statement_day: u8, due_day: u8, interest_rate: Option<String>,
) -> Result<(), String> {
    let s = s();
    s.rt.block_on(add_credit_card::execute(s.uow.as_ref(), add_credit_card::AddCreditCardCommand {
        owner_id: s.user_id, name, last_four, network, expiry_month, expiry_year,
        credit_limit:       dec(&credit_limit)?,
        currency:           cur(&currency)?,
        outstanding:        outstanding.as_deref().map(dec).transpose()?.unwrap_or(Decimal::ZERO),
        cash_advance_limit: cash_advance_limit.as_deref().map(dec).transpose()?,
        statement_day, due_day,
        interest_rate:      interest_rate.as_deref().map(dec).transpose()?,
    })).map_err(e)
}

pub fn open_loan_account(
    name: String, account_number: Option<String>, bank: String, creditor: String,
    currency: String, principal: String,
    interest_rate: Option<String>, due_day: Option<u8>,
    maturity_date: Option<String>, minimum_payment: Option<String>,
) -> Result<(), String> {
    let s = s();
    let maturity = maturity_date.as_deref()
        .map(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").map_err(|err| err.to_string()))
        .transpose()?;
    s.rt.block_on(open_loan_account::execute(s.uow.as_ref(), open_loan_account::OpenLoanAccountCommand {
        owner_id: s.user_id, name, account_number, bank, creditor,
        currency:        cur(&currency)?,
        principal:       dec(&principal)?,
        interest_rate:   interest_rate.as_deref().map(dec).transpose()?,
        due_day, maturity_date: maturity,
        minimum_payment: minimum_payment.as_deref().map(dec).transpose()?,
    })).map_err(e)
}

// ── Transactions ──────────────────────────────────────────────────────────────

pub fn deposit(account_id: String, amount: String, currency: String, label: Option<String>, description: Option<String>) -> Result<(), String> {
    let s = s();
    s.rt.block_on(deposit_funds::execute(s.uow.as_ref(), deposit_funds::DepositFundsCommand {
        owner_id: s.user_id, account_id: aid(&account_id)?,
        amount: dec(&amount)?, currency: cur(&currency)?, label, description,
    })).map_err(e)
}

pub fn withdraw(account_id: String, amount: String, currency: String, label: Option<String>, description: Option<String>) -> Result<(), String> {
    let s = s();
    s.rt.block_on(withdraw_funds::execute(s.uow.as_ref(), withdraw_funds::WithdrawFundsCommand {
        owner_id: s.user_id, account_id: aid(&account_id)?,
        amount: dec(&amount)?, currency: cur(&currency)?, label, description,
    })).map_err(e)
}

pub fn transfer(from: String, to: String, amount: String, currency: String, label: Option<String>, description: Option<String>) -> Result<(), String> {
    let s = s();
    s.rt.block_on(transfer_funds::execute(s.uow.as_ref(), transfer_funds::TransferFundsCommand {
        owner_id: s.user_id,
        from_account_id: aid(&from)?, to_account_id: aid(&to)?,
        amount: dec(&amount)?, currency: cur(&currency)?, label, description,
    })).map_err(e)
}

pub fn charge(account_id: String, amount: String, currency: String, label: Option<String>, description: Option<String>) -> Result<(), String> {
    let s = s();
    s.rt.block_on(charge_credit_card::execute(s.uow.as_ref(), charge_credit_card::ChargeCreditCardCommand {
        owner_id: s.user_id, account_id: aid(&account_id)?,
        amount: dec(&amount)?, currency: cur(&currency)?, label, description,
    })).map_err(e)
}

pub fn pay(account_id: String, from_account_id: String, amount: String, currency: String, label: Option<String>, description: Option<String>) -> Result<(), String> {
    let s = s();
    s.rt.block_on(make_payment::execute(
        s.uow.as_ref(),
        &*s.statement_repo,
        make_payment::MakePaymentCommand {
            owner_id:        s.user_id,
            debt_account_id: aid(&account_id)?,
            from_account_id: aid(&from_account_id)?,
            amount: dec(&amount)?, currency: cur(&currency)?, label, description,
        },
    )).map_err(e)
}

pub fn accrue_interest(account_id: String) -> Result<(), String> {
    let s = s();
    s.rt.block_on(accrue_interest::execute(s.uow.as_ref(), accrue_interest::AccrueInterestCommand {
        owner_id: s.user_id, account_id: aid(&account_id)?,
    })).map_err(e)
}

pub fn grant_limit(account_id: String, new_limit: String, currency: String, expires_on: String) -> Result<(), String> {
    let s   = s();
    let exp = chrono::NaiveDate::parse_from_str(&expires_on, "%Y-%m-%d").map_err(|err| err.to_string())?;
    s.rt.block_on(grant_temporary_limit::execute(s.uow.as_ref(), grant_temporary_limit::GrantTemporaryLimitCommand {
        owner_id: s.user_id, account_id: aid(&account_id)?,
        new_limit: dec(&new_limit)?, currency: cur(&currency)?, expires_on: exp,
    })).map_err(e)
}

pub fn revoke_limit(account_id: String) -> Result<(), String> {
    let s = s();
    s.rt.block_on(revoke_temporary_limit::execute(s.uow.as_ref(), revoke_temporary_limit::RevokeTemporaryLimitCommand {
        owner_id: s.user_id, account_id: aid(&account_id)?,
    })).map_err(e)
}

// ── Ledger entries ────────────────────────────────────────────────────────────

pub fn list_entries(account_id: String) -> Result<Vec<LedgerEntry>, String> {
    let s  = s();
    let id = aid(&account_id)?;
    s.rt.block_on(s.entry_repo.list_for_account(id))
        .map(|v| v.into_iter().map(|entry| LedgerEntry {
            id:          entry.id.to_string(),
            account_id:  entry.account_id.to_string(),
            entry_type:  entry.entry_type.to_string(),
            amount:      entry.amount,
            currency:    entry.currency,
            occurred_at: entry.occurred_at.to_rfc3339(),
            label:       entry.label,
            description: entry.description,
        }).collect())
        .map_err(|err| err.to_string())
}

pub fn update_entry_annotation(entry_id: String, label: Option<String>, description: Option<String>) -> Result<(), String> {
    let s  = s();
    let id = uid(&entry_id)?;
    s.rt.block_on(async {
        s.entry_repo.find(id).await?
            .ok_or_else(|| LedgerError::Validation(format!("Entry {id} not found")))?;
        s.entry_repo.update_annotation(id, label, description).await
    }).map_err(e)
}

// ── Statements ────────────────────────────────────────────────────────────────

pub fn list_statements(account_id: String) -> Result<Vec<Statement>, String> {
    let s  = s();
    let id = aid(&account_id)?;
    s.rt.block_on(s.statement_repo.list_for_account(id))
        .map(|v| v.into_iter().map(|st| Statement {
            id:                st.id.to_string(),
            account_id:        st.account_id.uuid().to_string(),
            cycle_start:       st.cycle_start.to_string(),
            cycle_end:         st.cycle_end.to_string(),
            statement_balance: st.statement_balance.to_string(),
            minimum_payment:   st.minimum_payment.map(|d| d.to_string()),
            total_charged:     st.total_charged.to_string(),
            total_paid:        st.total_paid.to_string(),
            is_settled:        st.is_settled,
        }).collect())
        .map_err(|err| err.to_string())
}

pub fn close_statement(account_id: String, minimum_payment: Option<String>, currency: String, label: Option<String>, description: Option<String>) -> Result<(), String> {
    let s = s();
    s.rt.block_on(close_statement_with_record::execute(s.uow.as_ref(), close_statement_with_record::CloseStatementWithRecordCommand {
        owner_id:        s.user_id,
        account_id:      aid(&account_id)?,
        minimum_payment: minimum_payment.as_deref().map(dec).transpose()?,
        currency:        cur(&currency)?,
        label, description,
    })).map_err(e)
}

// ── Holdings ──────────────────────────────────────────────────────────────────

pub fn add_holding(account_id: String, ticker: String, quantity: String, unit_price: String, currency: String, investment_type: String) -> Result<(), String> {
    let s = s();
    s.rt.block_on(add_holding::execute(s.uow.as_ref(), add_holding::AddHoldingCommand {
        owner_id:    s.user_id, account_id: aid(&account_id)?,
        ticker, investment_type,
        quantity:    dec(&quantity)?,
        unit_price:  dec(&unit_price)?,
        currency:    cur(&currency)?,
    })).map_err(e)
}

pub fn remove_holding(account_id: String, ticker: String) -> Result<(), String> {
    let s = s();
    s.rt.block_on(remove_holding::execute(s.uow.as_ref(), remove_holding::RemoveHoldingCommand {
        owner_id: s.user_id, account_id: aid(&account_id)?, ticker,
    })).map_err(e)
}

pub fn update_holding_price(account_id: String, ticker: String, new_price: String, currency: String) -> Result<(), String> {
    let s = s();
    s.rt.block_on(update_holding_price::execute(s.uow.as_ref(), update_holding_price::UpdateHoldingPriceCommand {
        owner_id: s.user_id, account_id: aid(&account_id)?,
        ticker, new_price: dec(&new_price)?, currency: cur(&currency)?,
    })).map_err(e)
}
