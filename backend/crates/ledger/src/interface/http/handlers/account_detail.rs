//! Rich per-type account detail endpoints.
//! Each returns exactly the fields meaningful for that account type.

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use rust_decimal::Decimal;
use serde::Serialize;
use uuid::Uuid;

use crate::application::error::LedgerError;
use crate::domain::account_id::AccountId;
use crate::domain::asset_account::AssetAccount;
use crate::domain::financial_entry::FinancialEntry;
use crate::domain::revolving_credit::RevolvingCredit;
use crate::interface::http::app_state::AppState;

// ── Cash account ──────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct CashAccountDetail {
    pub account_id:     String,
    pub account_name:   String,
    pub currency:       String,
    pub balance:        String,
    pub bank:           String,
    pub account_number: String,
}

pub async fn cash_detail(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, LedgerError> {
    let finances   = s.repo.load(s.user_id).await?;
    let account_id = AccountId::restore(id)?;

    let a = finances
        .find_account(account_id)
        .ok_or(LedgerError::AccountNotFound(account_id.uuid()))?
        .as_cash()
        .ok_or(LedgerError::WrongAccountType(account_id.uuid()))?;

    Ok(Json(CashAccountDetail {
        account_id:     a.account_id().to_string(),
        account_name:   a.account_name().value().to_string(),
        currency:       format!("{:?}", a.currency()),
        balance:        a.balance().amount().to_string(),
        bank:           a.bank().value().to_string(),
        account_number: a.account_number().value().to_string(),
    }))
}

// ── Physical wallet ───────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct PhysicalWalletDetail {
    pub account_id:   String,
    pub account_name: String,
    pub currency:     String,
    pub balance:      String,
}

pub async fn wallet_detail(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, LedgerError> {
    let finances   = s.repo.load(s.user_id).await?;
    let account_id = AccountId::restore(id)?;

    let a = finances
        .find_account(account_id)
        .ok_or(LedgerError::AccountNotFound(account_id.uuid()))?
        .as_physical_wallet()
        .ok_or(LedgerError::WrongAccountType(account_id.uuid()))?;

    Ok(Json(PhysicalWalletDetail {
        account_id:   a.account_id().to_string(),
        account_name: a.account_name().value().to_string(),
        currency:     format!("{:?}", a.currency()),
        balance:      a.balance().amount().to_string(),
    }))
}

// ── Digital wallet ────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct DigitalWalletDetail {
    pub account_id:          String,
    pub account_name:        String,
    pub currency:            String,
    pub balance:             String,
    pub provider:            String,
    pub provider_account_id: String,
}

pub async fn digital_wallet_detail(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, LedgerError> {
    let finances   = s.repo.load(s.user_id).await?;
    let account_id = AccountId::restore(id)?;

    let a = finances
        .find_account(account_id)
        .ok_or(LedgerError::AccountNotFound(account_id.uuid()))?
        .as_digital_wallet()
        .ok_or(LedgerError::WrongAccountType(account_id.uuid()))?;

    Ok(Json(DigitalWalletDetail {
        account_id:          a.account_id().to_string(),
        account_name:        a.account_name().value().to_string(),
        currency:            format!("{:?}", a.currency()),
        balance:             a.balance().amount().to_string(),
        provider:            a.provider().to_string(),
        provider_account_id: a.provider_account_id().value().to_string(),
    }))
}

// ── Investment account ────────────────────────────────────────────────────────

#[derive(Serialize)]
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

#[derive(Serialize)]
pub struct HoldingDetail {
    pub ticker:          String,
    pub investment_type: String,
    pub quantity:        String,
    pub unit_price:      String,
    pub market_value:    String,
    pub currency:        String,
}

pub async fn investment_detail(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, LedgerError> {
    let finances   = s.repo.load(s.user_id).await?;
    let account_id = AccountId::restore(id)?;

    let a = finances
        .find_account(account_id)
        .ok_or(LedgerError::AccountNotFound(account_id.uuid()))?
        .as_investment()
        .ok_or(LedgerError::WrongAccountType(account_id.uuid()))?;

    let holdings_value = a.holdings_value()
        .map(|m| m.amount().to_string())
        .unwrap_or_else(|_| "0".to_string());

    let total_value = a.total_value()
        .map(|m| m.amount().to_string())
        .unwrap_or_else(|_| a.cash_balance().amount().to_string());

    let holdings = a.holdings().iter().map(|h| {
        let market_value = h.market_value()
            .map(|m| m.amount().to_string())
            .unwrap_or_else(|_| "0".to_string());
        HoldingDetail {
            ticker:          h.ticker().value().to_string(),
            investment_type: format!("{:?}", h.investment_type()),
            quantity:        h.quantity().to_string(),
            unit_price:      h.unit_price().amount().to_string(),
            market_value,
            currency:        format!("{:?}", h.unit_price().currency()),
        }
    }).collect();

    Ok(Json(InvestmentAccountDetail {
        account_id:     a.account_id().to_string(),
        account_name:   a.account_name().value().to_string(),
        currency:       format!("{:?}", a.currency()),
        cash_balance:   a.cash_balance().amount().to_string(),
        holdings_value,
        total_value,
        bank:           a.bank().value().to_string(),
        account_number: a.account_number().value().to_string(),
        holdings,
    }))
}

// ── Loan account ──────────────────────────────────────────────────────────────

#[derive(Serialize)]
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
    pub interest_rate:   Option<Decimal>,
    pub due_day:         Option<u8>,
    pub maturity_date:   Option<String>,
    pub minimum_payment: Option<String>,
    pub is_overdue:      bool,
    pub is_settled:      bool,
}

pub async fn loan_detail(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, LedgerError> {
    let finances   = s.repo.load(s.user_id).await?;
    let account_id = AccountId::restore(id)?;

    let a = finances
        .find_account(account_id)
        .ok_or(LedgerError::AccountNotFound(account_id.uuid()))?
        .as_loan()
        .ok_or(LedgerError::WrongAccountType(account_id.uuid()))?;

    let loan        = a.loan();
    let principal   = loan.principal().amount();
    let outstanding = loan.outstanding().amount();
    let paid        = principal - outstanding;

    let percent_paid = if principal.is_zero() {
        "0.00".to_string()
    } else {
        format!("{:.2}", (paid / principal * rust_decimal::Decimal::ONE_HUNDRED))
    };

    Ok(Json(LoanAccountDetail {
        account_id:      a.account_id().to_string(),
        account_name:    a.account_name().value().to_string(),
        currency:        format!("{:?}", a.currency()),
        bank:            a.bank().value().to_string(),
        account_number:  a.account_number().map(|n| n.value().to_string()),
        creditor:        loan.creditor().to_string(),
        principal:       principal.to_string(),
        outstanding:     outstanding.to_string(),
        amount_paid:     paid.to_string(),
        percent_paid,
        interest_rate:   loan.interest_rate(),
        due_day:         loan.due_date().map(|d| d.value()),
        maturity_date:   loan.maturity_date().map(|d| d.to_string()),
        minimum_payment: loan.minimum_payment().map(|l| l.amount().to_string()),
        is_overdue:      loan.is_overdue(),
        is_settled:      loan.is_settled(),
    }))
}
