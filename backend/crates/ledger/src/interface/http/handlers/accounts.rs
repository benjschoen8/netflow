use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::error::LedgerError;
use crate::application::use_cases::{
    create_user_finances, list_accounts, get_net_worth,
    open_cash_account, add_physical_wallet, add_digital_wallet,
    open_investment_account, add_credit_card, open_loan_account,
    remove_account,
};
use crate::domain::account_id::AccountId;
use crate::domain::currency::Currency;
use crate::interface::http::app_state::AppState;

// ── Init ──────────────────────────────────────────────────────────────────────

pub async fn init(State(s): State<AppState>) -> Result<impl IntoResponse, LedgerError> {
    create_user_finances::execute(
        s.repo.as_ref(),
        create_user_finances::CreateUserFinancesCommand { owner_id: s.user_id },
    ).await?;
    Ok(StatusCode::CREATED)
}

// ── List ──────────────────────────────────────────────────────────────────────

pub async fn list(State(s): State<AppState>) -> Result<impl IntoResponse, LedgerError> {
    let accounts = list_accounts::execute(
        s.repo.as_ref(),
        list_accounts::ListAccountsQuery { owner_id: s.user_id },
    ).await?;
    Ok(Json(accounts))
}

// ── Net worth ─────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct NetWorthParams {
    currency: Option<String>,
}

pub async fn net_worth(
    State(s): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<NetWorthParams>,
) -> Result<impl IntoResponse, LedgerError> {
    let currency = params.currency.as_deref().map(parse_currency).transpose()?;
    let result = get_net_worth::execute(
        s.repo.as_ref(),
        get_net_worth::GetNetWorthQuery { owner_id: s.user_id, currency },
    ).await?;
    Ok(Json(result))
}

// ── Open cash account ─────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct OpenCashAccountRequest {
    pub name:            String,
    pub account_number:  String,
    pub bank:            String,
    pub currency:        String,
    pub initial_balance: Decimal,
}

pub async fn open_cash(
    State(s): State<AppState>,
    Json(req): Json<OpenCashAccountRequest>,
) -> Result<impl IntoResponse, LedgerError> {
    open_cash_account::execute(s.repo.as_ref(), open_cash_account::OpenCashAccountCommand {
        owner_id:        s.user_id,
        name:            req.name,
        account_number:  req.account_number,
        bank:            req.bank,
        currency:        parse_currency(&req.currency)?,
        initial_balance: req.initial_balance,
    }).await?;
    Ok(StatusCode::CREATED)
}

// ── Add physical wallet ───────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct AddWalletRequest {
    pub name:            String,
    pub currency:        String,
    pub initial_balance: Decimal,
}

pub async fn add_wallet(
    State(s): State<AppState>,
    Json(req): Json<AddWalletRequest>,
) -> Result<impl IntoResponse, LedgerError> {
    add_physical_wallet::execute(s.repo.as_ref(), add_physical_wallet::AddPhysicalWalletCommand {
        owner_id:        s.user_id,
        name:            req.name,
        currency:        parse_currency(&req.currency)?,
        initial_balance: req.initial_balance,
    }).await?;
    Ok(StatusCode::CREATED)
}

// ── Add digital wallet ────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct AddDigitalWalletRequest {
    pub name:                String,
    pub provider:            String,
    pub provider_account_id: String,
    pub currency:            String,
    pub initial_balance:     Decimal,
}

pub async fn add_digital_wallet_handler(
    State(s): State<AppState>,
    Json(req): Json<AddDigitalWalletRequest>,
) -> Result<impl IntoResponse, LedgerError> {
    add_digital_wallet::execute(s.repo.as_ref(), add_digital_wallet::AddDigitalWalletCommand {
        owner_id:            s.user_id,
        name:                req.name,
        provider:            req.provider,
        provider_account_id: req.provider_account_id,
        currency:            parse_currency(&req.currency)?,
        initial_balance:     req.initial_balance,
    }).await?;
    Ok(StatusCode::CREATED)
}

// ── Open investment account ───────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct OpenInvestmentRequest {
    pub name:           String,
    pub account_number: String,
    pub bank:           String,
    pub currency:       String,
    pub cash_balance:   Decimal,
}

pub async fn open_investment(
    State(s): State<AppState>,
    Json(req): Json<OpenInvestmentRequest>,
) -> Result<impl IntoResponse, LedgerError> {
    open_investment_account::execute(s.repo.as_ref(), open_investment_account::OpenInvestmentAccountCommand {
        owner_id:       s.user_id,
        name:           req.name,
        account_number: req.account_number,
        bank:           req.bank,
        currency:       parse_currency(&req.currency)?,
        cash_balance:   req.cash_balance,
    }).await?;
    Ok(StatusCode::CREATED)
}

// ── Add credit card ───────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct AddCreditCardRequest {
    pub name:               String,
    pub last_four:          String,
    pub network:            String,
    pub expiry_month:       u8,
    pub expiry_year:        u16,
    pub credit_limit:       Decimal,
    pub currency:           String,
    pub outstanding:        Option<Decimal>,
    pub cash_advance_limit: Option<Decimal>,
    pub statement_day:      u8,
    pub due_day:            u8,
    pub interest_rate:      Option<Decimal>,
}

pub async fn add_credit_card_handler(
    State(s): State<AppState>,
    Json(req): Json<AddCreditCardRequest>,
) -> Result<impl IntoResponse, LedgerError> {
    add_credit_card::execute(s.repo.as_ref(), add_credit_card::AddCreditCardCommand {
        owner_id:           s.user_id,
        name:               req.name,
        last_four:          req.last_four,
        network:            req.network,
        expiry_month:       req.expiry_month,
        expiry_year:        req.expiry_year,
        credit_limit:       req.credit_limit,
        currency:           parse_currency(&req.currency)?,
        outstanding:        req.outstanding.unwrap_or(Decimal::ZERO),
        cash_advance_limit: req.cash_advance_limit,
        statement_day:      req.statement_day,
        due_day:            req.due_day,
        interest_rate:      req.interest_rate,
    }).await?;
    Ok(StatusCode::CREATED)
}

// ── Open loan account ─────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct OpenLoanRequest {
    pub name:            String,
    pub account_number:  Option<String>,
    pub bank:            String,
    pub creditor:        String,
    pub currency:        String,
    pub principal:       Decimal,
    pub interest_rate:   Option<Decimal>,
    pub due_day:         Option<u8>,
    pub maturity_date:   Option<String>,
    pub minimum_payment: Option<Decimal>,
}

pub async fn open_loan(
    State(s): State<AppState>,
    Json(req): Json<OpenLoanRequest>,
) -> Result<impl IntoResponse, LedgerError> {
    let maturity_date = req.maturity_date
        .as_deref()
        .map(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map_err(|_| LedgerError::Validation(format!("Invalid date: {}", s))))
        .transpose()?;

    open_loan_account::execute(s.repo.as_ref(), open_loan_account::OpenLoanAccountCommand {
        owner_id:        s.user_id,
        name:            req.name,
        account_number:  req.account_number,
        bank:            req.bank,
        creditor:        req.creditor,
        currency:        parse_currency(&req.currency)?,
        principal:       req.principal,
        interest_rate:   req.interest_rate,
        due_day:         req.due_day,
        maturity_date,
        minimum_payment: req.minimum_payment,
    }).await?;
    Ok(StatusCode::CREATED)
}

// ── Remove account ────────────────────────────────────────────────────────────

pub async fn remove(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, LedgerError> {
    remove_account::execute(s.repo.as_ref(), remove_account::RemoveAccountCommand {
        owner_id:   s.user_id,
        account_id: AccountId::restore(id)?,
    }).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ── Helper ────────────────────────────────────────────────────────────────────

pub fn parse_currency(s: &str) -> Result<Currency, LedgerError> {
    match s.to_uppercase().as_str() {
        "TWD" => Ok(Currency::TWD),
        "USD" => Ok(Currency::USD),
        other => Err(LedgerError::Validation(format!("Unknown currency: {}", other))),
    }
}
