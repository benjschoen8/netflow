//! JSON serialisation bridge between `FinancialAccount` (domain) and SQLite.
//!
//! We use a `#[serde(tag = "type")]` enum so every stored JSON object carries
//! its discriminant, making the array self-describing and migration-friendly.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared::domain::SharedError;

// ── Trait imports — required to call trait methods on concrete domain types ───
use crate::domain::asset_account::AssetAccount;
use crate::domain::debt_account::DebtAccount;
use crate::domain::financial_entry::FinancialEntry;

use crate::domain::{
    account_id::AccountId,
    account_name::AccountName,
    account_number::AccountNumber,
    bank::Bank,
    card_last_four::CardLastFour,
    card_network::CardNetwork,
    cash_account::CashAccount,
    credit_card::CreditCard,
    currency::Currency,
    digital_wallet::{DigitalWallet, ProviderAccountId},
    digital_wallet_provider::DigitalWalletProvider,
    expiration_date::ExpirationDate,
    financial_account::FinancialAccount,
    investment::Investment,
    investment_account::InvestmentAccount,
    investment_type::InvestmentType,
    liability::Liability,
    loan::Loan,
    loan_account::LoanAccount,
    money::Money,
    monthly_day::MonthlyDay,
    physical_wallet::PhysicalWallet,
    ticker::Ticker,
};

// ── Leaf value-object helpers ─────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
struct MoneyRow {
    amount:   Decimal,
    currency: Currency,
}

impl MoneyRow {
    fn from_domain(m: &Money) -> Self {
        Self { amount: m.amount(), currency: m.currency() }
    }
    fn into_domain(self) -> Result<Money, SharedError> {
        Money::new(self.amount, self.currency)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct LiabilityRow {
    amount:   Decimal,
    currency: Currency,
}

impl LiabilityRow {
    fn from_domain(l: &Liability) -> Self {
        Self { amount: l.amount(), currency: l.currency() }
    }
    fn into_domain(self) -> Result<Liability, SharedError> {
        Liability::new(self.amount, self.currency)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct InvestmentRow {
    ticker:          String,
    investment_type: InvestmentType,
    quantity:        Decimal,
    unit_price:      MoneyRow,
}

impl InvestmentRow {
    fn from_domain(i: &Investment) -> Self {
        Self {
            ticker:          i.ticker().value().to_string(),
            investment_type: i.investment_type().clone(),
            quantity:        i.quantity(),
            unit_price:      MoneyRow::from_domain(i.unit_price()),
        }
    }
    fn into_domain(self) -> Result<Investment, SharedError> {
        Investment::new(
            Ticker::new(self.ticker)?,
            self.investment_type,
            self.quantity,
            self.unit_price.into_domain()?,
        )
    }
}

// ── Per-variant row types ─────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
struct CashAccountRow {
    account_id:     Uuid,
    account_name:   String,
    account_number: String,
    bank:           String,
    balance:        MoneyRow,
}

#[derive(Debug, Serialize, Deserialize)]
struct PhysicalWalletRow {
    account_id:   Uuid,
    account_name: String,
    balance:      MoneyRow,
}

#[derive(Debug, Serialize, Deserialize)]
struct DigitalWalletRow {
    account_id:          Uuid,
    account_name:        String,
    provider:            DigitalWalletProvider,
    provider_account_id: String,
    balance:             MoneyRow,
}

#[derive(Debug, Serialize, Deserialize)]
struct InvestmentAccountRow {
    account_id:     Uuid,
    account_name:   String,
    account_number: String,
    bank:           String,
    cash_balance:   MoneyRow,
    holdings:       Vec<InvestmentRow>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoanRow {
    principal:       LiabilityRow,
    creditor:        String,
    interest_rate:   Option<Decimal>,
    due_day:         Option<u8>,
    maturity_date:   Option<String>, // ISO-8601
    minimum_payment: Option<LiabilityRow>,
    overdue:         bool,
    outstanding:     LiabilityRow,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoanAccountRow {
    account_id:     Uuid,
    account_name:   String,
    account_number: Option<String>,
    bank:           String,
    loan:           LoanRow,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemporaryCreditLimitRow {
    amount:     MoneyRow,
    expires_on: String, // ISO-8601
}

#[derive(Debug, Serialize, Deserialize)]
struct CreditCardRow {
    account_id:           Uuid,
    account_name:         String,
    last_four:            String,
    network:              CardNetwork,
    expiry_month:         u8,
    expiry_year:          u16,
    credit_limit:         MoneyRow,
    temporary_credit_limit: Option<TemporaryCreditLimitRow>,
    cash_advance_limit:   Option<MoneyRow>,
    outstanding_balance:  LiabilityRow,
    statement_balance:    Option<LiabilityRow>,
    statement_day:        u8,
    due_day:              u8,
    interest_rate:        Option<Decimal>,
    minimum_payment:      Option<LiabilityRow>,
    minimum_payment_paid: bool,
    is_paid:              bool,
}

// ── Top-level tagged enum ─────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum StoredAccount {
    Cash(CashAccountRow),
    PhysicalWallet(PhysicalWalletRow),
    DigitalWallet(DigitalWalletRow),
    Investment(InvestmentAccountRow),
    Loan(LoanAccountRow),
    CreditCard(CreditCardRow),
}

// ── FinancialAccount → StoredAccount ─────────────────────────────────────────

impl StoredAccount {
    pub fn from_domain(account: &FinancialAccount) -> Self {
        match account {
            FinancialAccount::Cash(a) => StoredAccount::Cash(CashAccountRow {
                account_id:     a.account_id().uuid(),
                account_name:   a.account_name().value().to_string(),
                account_number: a.account_number().value().to_string(),
                bank:           a.bank().value().to_string(),
                balance:        MoneyRow::from_domain(a.balance()),
            }),
            FinancialAccount::PhysicalWallet(a) => StoredAccount::PhysicalWallet(PhysicalWalletRow {
                account_id:   a.account_id().uuid(),
                account_name: a.account_name().value().to_string(),
                balance:      MoneyRow::from_domain(a.balance()),
            }),
            FinancialAccount::DigitalWallet(a) => StoredAccount::DigitalWallet(DigitalWalletRow {
                account_id:          a.account_id().uuid(),
                account_name:        a.account_name().value().to_string(),
                provider:            a.provider().clone(),
                provider_account_id: a.provider_account_id().value().to_string(),
                balance:             MoneyRow::from_domain(a.balance()),
            }),
            FinancialAccount::Investment(a) => StoredAccount::Investment(InvestmentAccountRow {
                account_id:     a.account_id().uuid(),
                account_name:   a.account_name().value().to_string(),
                account_number: a.account_number().value().to_string(),
                bank:           a.bank().value().to_string(),
                cash_balance:   MoneyRow::from_domain(a.cash_balance()),
                holdings:       a.holdings().iter().map(InvestmentRow::from_domain).collect(),
            }),
            FinancialAccount::Loan(a) => {
                let l = a.loan();
                StoredAccount::Loan(LoanAccountRow {
                    account_id:     a.account_id().uuid(),
                    account_name:   a.account_name().value().to_string(),
                    account_number: a.account_number().map(|n| n.value().to_string()),
                    bank:           a.bank().value().to_string(),
                    loan: LoanRow {
                        principal:       LiabilityRow::from_domain(l.principal()),
                        creditor:        l.creditor().to_string(),
                        interest_rate:   l.interest_rate(),
                        due_day:         l.due_date().map(|d| d.value()),
                        maturity_date:   l.maturity_date().map(|d| d.to_string()),
                        minimum_payment: l.minimum_payment().map(LiabilityRow::from_domain),
                        overdue:         l.is_overdue(),
                        outstanding:     LiabilityRow::from_domain(l.outstanding()),
                    },
                })
            }
            FinancialAccount::CreditCard(a) => {
                let temp = a.temporary_credit_limit().map(|t| TemporaryCreditLimitRow {
                    amount:     MoneyRow::from_domain(t.amount()),
                    expires_on: t.expires_on().to_string(),
                });
                StoredAccount::CreditCard(CreditCardRow {
                    account_id:             a.account_id().uuid(),
                    account_name:           a.account_name().value().to_string(),
                    last_four:              a.card_last_four().value().to_string(),
                    network:                a.network().clone(),
                    expiry_month:           a.expiration_date().month(),
                    expiry_year:            a.expiration_date().year(),
                    credit_limit:           MoneyRow::from_domain(a.credit_limit()),
                    temporary_credit_limit: temp,
                    cash_advance_limit:     a.cash_advance_limit().map(MoneyRow::from_domain),
                    outstanding_balance:    LiabilityRow::from_domain(a.outstanding()),
                    statement_balance:      a.statement_balance().map(LiabilityRow::from_domain),
                    statement_day:          a.statement_date().value(),
                    due_day:                a.payment_due_date().value(),
                    interest_rate:          a.interest_rate(),
                    minimum_payment:        a.minimum_payment().map(LiabilityRow::from_domain),
                    minimum_payment_paid:   a.minimum_payment_paid(),
                    is_paid:                a.is_paid(),
                })
            }
        }
    }

    pub fn into_domain(self) -> Result<FinancialAccount, SharedError> {
        match self {
            StoredAccount::Cash(r) => Ok(FinancialAccount::Cash(CashAccount::new(
                AccountId::restore(r.account_id)?,
                AccountName::new(r.account_name)?,
                AccountNumber::new(r.account_number)?,
                Bank::new(r.bank)?,
                r.balance.into_domain()?,
            ))),
            StoredAccount::PhysicalWallet(r) => Ok(FinancialAccount::PhysicalWallet(
                PhysicalWallet::new(
                    AccountId::restore(r.account_id)?,
                    AccountName::new(r.account_name)?,
                    r.balance.into_domain()?,
                ),
            )),
            StoredAccount::DigitalWallet(r) => Ok(FinancialAccount::DigitalWallet(
                DigitalWallet::new(
                    AccountId::restore(r.account_id)?,
                    AccountName::new(r.account_name)?,
                    r.provider,
                    ProviderAccountId::new(r.provider_account_id)?,
                    r.balance.into_domain()?,
                ),
            )),
            StoredAccount::Investment(r) => {
                let mut acc = InvestmentAccount::new(
                    AccountId::restore(r.account_id)?,
                    AccountName::new(r.account_name)?,
                    AccountNumber::new(r.account_number)?,
                    Bank::new(r.bank)?,
                    r.cash_balance.into_domain()?,
                );
                for h in r.holdings {
                    acc.add_holding(h.into_domain()?)?;
                }
                Ok(FinancialAccount::Investment(acc))
            }
            StoredAccount::Loan(r) => {
                let minimum    = r.loan.minimum_payment.map(|m| m.into_domain()).transpose()?;
                let due_day    = r.loan.due_day.map(MonthlyDay::new).transpose()?;
                let maturity   = r.loan.maturity_date
                    .map(|s| s.parse::<chrono::NaiveDate>()
                        .map_err(|_| SharedError::InvalidFormat("[LoanRow] bad maturity_date format")))
                    .transpose()?;
                let principal   = r.loan.principal.into_domain()?;
                let outstanding = r.loan.outstanding.into_domain()?;
                let loan = Loan::restore(
                    principal,
                    outstanding,
                    r.loan.creditor,
                    r.loan.interest_rate,
                    due_day,
                    maturity,
                    minimum,
                    r.loan.overdue,
                )?;
                let account_number = r.account_number.map(AccountNumber::new).transpose()?;
                Ok(FinancialAccount::Loan(LoanAccount::new(
                    AccountId::restore(r.account_id)?,
                    AccountName::new(r.account_name)?,
                    account_number,
                    Bank::new(r.bank)?,
                    loan,
                )))
            }
            StoredAccount::CreditCard(r) => {
                let cash_advance = r.cash_advance_limit.map(|m| m.into_domain()).transpose()?;
                let mut card = CreditCard::new(
                    AccountId::restore(r.account_id)?,
                    AccountName::new(r.account_name)?,
                    CardLastFour::new(r.last_four)?,
                    r.network,
                    ExpirationDate::new(r.expiry_month, r.expiry_year)?,
                    r.credit_limit.into_domain()?,
                    cash_advance,
                    r.outstanding_balance.into_domain()?,
                    MonthlyDay::new(r.statement_day)?,
                    MonthlyDay::new(r.due_day)?,
                    r.interest_rate,
                )?;
                if let Some(stmt) = r.statement_balance {
                    card.restore_statement_balance(stmt.into_domain()?);
                }
                if let Some(min) = r.minimum_payment {
                    card.restore_minimum_payment(min.into_domain()?);
                }
                if r.minimum_payment_paid {
                    card.mark_current();
                }
                if let Some(temp) = r.temporary_credit_limit {
                    let expires = temp.expires_on.parse::<chrono::NaiveDate>()
                        .map_err(|_| SharedError::InvalidFormat("[CreditCardRow] bad temp limit date"))?;
                    card.grant_temporary_limit(temp.amount.into_domain()?, expires)?;
                }
                Ok(FinancialAccount::CreditCard(card))
            }
        }
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Serialize the full accounts list to a JSON string.
pub fn accounts_to_json(accounts: &[FinancialAccount]) -> Result<String, SharedError> {
    let rows: Vec<StoredAccount> = accounts
        .iter()
        .map(StoredAccount::from_domain)
        .collect();
    serde_json::to_string(&rows)
        .map_err(|e| SharedError::Serialization(e.to_string()))
}

/// Deserialize the accounts JSON string back to domain objects.
pub fn accounts_from_json(json: &str) -> Result<Vec<FinancialAccount>, SharedError> {
    let rows: Vec<StoredAccount> = serde_json::from_str(json)
        .map_err(|e| SharedError::Serialization(e.to_string()))?;
    rows.into_iter().map(StoredAccount::into_domain).collect()
}
