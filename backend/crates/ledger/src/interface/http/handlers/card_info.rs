//! Credit card detail endpoint — returns richer data than the generic account summary.

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
use crate::domain::revolving_credit::RevolvingCredit;
use crate::domain::debt_account::DebtAccount;
use crate::domain::financial_entry::FinancialEntry;
use crate::interface::http::app_state::AppState;

#[derive(Serialize)]
pub struct CreditCardInfo {
    pub account_id:           String,
    pub account_name:         String,
    pub currency:             String,
    pub network:              String,
    pub last_four:            String,
    pub expiry:               String,          // "MM/YY"
    pub credit_limit:         String,
    pub available_credit:     String,
    pub outstanding:          String,
    pub statement_day:        u8,              // day of month statement closes
    pub due_day:              u8,              // day of month payment is due
    pub interest_rate:        Option<Decimal>,
    pub minimum_payment:      Option<String>,
    pub minimum_payment_paid: bool,
    pub is_overdue:           bool,
    pub is_paid:              bool,
    pub statement_balance:    Option<String>,  // last closed statement amount
    pub temp_limit:           Option<TempLimitInfo>,
}

#[derive(Serialize)]
pub struct TempLimitInfo {
    pub amount:     String,
    pub expires_on: String,  // YYYY-MM-DD
}

/// `GET /accounts/:id/credit-card`
pub async fn get_card_info(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, LedgerError> {
    let finances   = s.repo.load(s.user_id).await?;
    let account_id = AccountId::restore(id)?;

    let card = finances
        .find_account(account_id)
        .ok_or(LedgerError::AccountNotFound(account_id.uuid()))?
        .as_credit_card()
        .ok_or(LedgerError::WrongAccountType(account_id.uuid()))?;

    let available = card.available_credit()
        .map(|m| m.amount().to_string())
        .unwrap_or_else(|_| "0".to_string());

    let temp_limit = card.temporary_credit_limit().map(|t| TempLimitInfo {
        amount:     t.amount().amount().to_string(),
        expires_on: t.expires_on().to_string(),
    });

    let info = CreditCardInfo {
        account_id:           card.account_id().to_string(),
        account_name:         card.account_name().value().to_string(),
        currency:             format!("{:?}", card.currency()),
        network:              card.network().to_string(),
        last_four:            card.card_last_four().value().to_string(),
        expiry:               card.expiration_date().to_string(),
        credit_limit:         card.credit_limit().amount().to_string(),
        available_credit:     available,
        outstanding:          card.outstanding().amount().to_string(),
        statement_day:        card.statement_date().value(),
        due_day:              card.payment_due_date().value(),
        interest_rate:        card.interest_rate(),
        minimum_payment:      card.minimum_payment().map(|l| l.amount().to_string()),
        minimum_payment_paid: card.minimum_payment_paid(),
        is_overdue:           card.is_overdue(),
        is_paid:              card.is_paid(),
        statement_balance:    card.statement_balance().map(|l| l.amount().to_string()),
        temp_limit,
    };

    Ok(Json(info))
}
