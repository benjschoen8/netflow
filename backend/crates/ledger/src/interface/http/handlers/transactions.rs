use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Deserialize;
use uuid::Uuid;

use crate::application::error::LedgerError;
use std::str::FromStr;
use crate::domain::currency::Currency;
use crate::application::use_cases::{
    accrue_interest, charge_credit_card, close_statement_with_record,
    deposit_funds, grant_temporary_limit, make_payment,
    revoke_temporary_limit, withdraw_funds,
};
use crate::domain::account_id::AccountId;
use crate::interface::http::app_state::AppState;


// ── Request types ─────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct AmountRequest {
    pub amount:      Decimal,
    pub currency:    String,
    pub label:       Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct PaymentRequest {
    pub from_account_id: Uuid,
    pub amount:          Decimal,
    pub currency:        String,
    pub label:           Option<String>,
    pub description:     Option<String>,
}

#[derive(Deserialize)]
pub struct CloseStatementRequest {
    pub minimum_payment: Option<Decimal>,
    pub currency:        String,
    pub label:           Option<String>,
    pub description:     Option<String>,
}

#[derive(Deserialize)]
pub struct GrantLimitRequest {
    pub new_limit:  Decimal,
    pub currency:   String,
    pub expires_on: String,
}

// ── Deposit ───────────────────────────────────────────────────────────────────

pub async fn deposit(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<AmountRequest>,
) -> Result<impl IntoResponse, LedgerError> {
    deposit_funds::execute(
        s.uow.as_ref(),
        deposit_funds::DepositFundsCommand {
            owner_id:    s.user_id,
            account_id:  AccountId::restore(id)?,
            amount:      req.amount,
            currency:    req.currency.parse::<Currency>().map_err(LedgerError::Validation)?,
            label:       req.label,
            description: req.description,
        },
    ).await?;
    Ok(StatusCode::OK)
}

// ── Withdraw ──────────────────────────────────────────────────────────────────

pub async fn withdraw(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<AmountRequest>,
) -> Result<impl IntoResponse, LedgerError> {
    withdraw_funds::execute(
        s.uow.as_ref(),
        withdraw_funds::WithdrawFundsCommand {
            owner_id:    s.user_id,
            account_id:  AccountId::restore(id)?,
            amount:      req.amount,
            currency:    req.currency.parse::<Currency>().map_err(LedgerError::Validation)?,
            label:       req.label,
            description: req.description,
        },
    ).await?;
    Ok(StatusCode::OK)
}

// ── Charge (credit card) ──────────────────────────────────────────────────────

pub async fn charge(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<AmountRequest>,
) -> Result<impl IntoResponse, LedgerError> {
    charge_credit_card::execute(
        s.uow.as_ref(),
        charge_credit_card::ChargeCreditCardCommand {
            owner_id:    s.user_id,
            account_id:  AccountId::restore(id)?,
            amount:      req.amount,
            currency:    req.currency.parse::<Currency>().map_err(LedgerError::Validation)?,
            label:       req.label,
            description: req.description,
        },
    ).await?;
    Ok(StatusCode::OK)
}

// ── Make payment (asset → debt) ───────────────────────────────────────────────

pub async fn pay(
    State(s): State<AppState>,
    Path(debt_id): Path<Uuid>,
    Json(req): Json<PaymentRequest>,
) -> Result<impl IntoResponse, LedgerError> {
    make_payment::execute(
        s.uow.as_ref(),
        s.statement_repo.as_ref(),
        make_payment::MakePaymentCommand {
            owner_id:        s.user_id,
            from_account_id: AccountId::restore(req.from_account_id)?,
            debt_account_id: AccountId::restore(debt_id)?,
            amount:          req.amount,
            currency:        req.currency.parse::<Currency>().map_err(LedgerError::Validation)?,
            label:           req.label,
            description:     req.description,
        },
    ).await?;
    Ok(StatusCode::OK)
}

// ── Close statement ───────────────────────────────────────────────────────────

pub async fn close_statement_handler(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CloseStatementRequest>,
) -> Result<impl IntoResponse, LedgerError> {
    close_statement_with_record::execute(
        s.uow.as_ref(),
        s.entry_repo.as_ref(),
        s.statement_repo.as_ref(),
        close_statement_with_record::CloseStatementWithRecordCommand {
            owner_id:        s.user_id,
            account_id:      AccountId::restore(id)?,
            minimum_payment: req.minimum_payment,
            currency:        req.currency.parse::<Currency>().map_err(LedgerError::Validation)?,
            label:           req.label,
            description:     req.description,
        },
    ).await?;
    Ok(StatusCode::OK)
}

// ── Grant temporary credit limit ──────────────────────────────────────────────

pub async fn grant_limit(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<GrantLimitRequest>,
) -> Result<impl IntoResponse, LedgerError> {
    grant_temporary_limit::execute(
        s.uow.as_ref(),
        grant_temporary_limit::GrantTemporaryLimitCommand {
            owner_id:   s.user_id,
            account_id: AccountId::restore(id)?,
            new_limit:  req.new_limit,
            currency:   req.currency.parse::<Currency>().map_err(LedgerError::Validation)?,
            expires_on: NaiveDate::parse_from_str(&req.expires_on, "%Y-%m-%d")
                .map_err(|_| LedgerError::Validation(
                    format!("Invalid date '{}'. Expected YYYY-MM-DD.", req.expires_on)
                ))?,
        },
    ).await?;
    Ok(StatusCode::OK)
}

// ── Revoke temporary credit limit ─────────────────────────────────────────────

pub async fn revoke_limit(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, LedgerError> {
    revoke_temporary_limit::execute(
        s.uow.as_ref(),
        revoke_temporary_limit::RevokeTemporaryLimitCommand {
            owner_id:   s.user_id,
            account_id: AccountId::restore(id)?,
        },
    ).await?;
    Ok(StatusCode::OK)
}

// ── Accrue interest ───────────────────────────────────────────────────────────

pub async fn accrue_interest_handler(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, LedgerError> {
    accrue_interest::execute(
        s.uow.as_ref(),
        accrue_interest::AccrueInterestCommand {
            owner_id:   s.user_id,
            account_id: AccountId::restore(id)?,
        },
    ).await?;
    Ok(StatusCode::OK)
}
