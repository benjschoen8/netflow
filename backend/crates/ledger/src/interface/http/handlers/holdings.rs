use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use rust_decimal::Decimal;
use serde::Deserialize;
use uuid::Uuid;

use crate::application::error::LedgerError;
use crate::application::use_cases::{add_holding, remove_holding, update_holding_price};
use crate::domain::account_id::AccountId;
use crate::interface::http::app_state::AppState;
use super::accounts::parse_currency;

// ── Add holding ───────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct AddHoldingRequest {
    pub ticker:          String,
    pub investment_type: String,  // "stock", "etf", "bond", "crypto", etc.
    pub quantity:        Decimal,
    pub unit_price:      Decimal,
    pub currency:        String,
}

pub async fn add(
    State(s): State<AppState>,
    Path(account_id): Path<Uuid>,
    Json(req): Json<AddHoldingRequest>,
) -> Result<impl IntoResponse, LedgerError> {
    add_holding::execute(s.repo.as_ref(), add_holding::AddHoldingCommand {
        owner_id:        s.user_id,
        account_id:      AccountId::restore(account_id)?,
        ticker:          req.ticker,
        investment_type: req.investment_type,
        quantity:        req.quantity,
        unit_price:      req.unit_price,
        currency:        parse_currency(&req.currency)?,
    }).await?;
    Ok(StatusCode::CREATED)
}

// ── Remove holding ────────────────────────────────────────────────────────────

pub async fn remove(
    State(s): State<AppState>,
    Path((account_id, ticker)): Path<(Uuid, String)>,
) -> Result<impl IntoResponse, LedgerError> {
    remove_holding::execute(s.repo.as_ref(), remove_holding::RemoveHoldingCommand {
        owner_id:   s.user_id,
        account_id: AccountId::restore(account_id)?,
        ticker,
    }).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ── Update holding price ──────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct UpdatePriceRequest {
    pub new_price: Decimal,
    pub currency:  String,
}

pub async fn update_price(
    State(s): State<AppState>,
    Path((account_id, ticker)): Path<(Uuid, String)>,
    Json(req): Json<UpdatePriceRequest>,
) -> Result<impl IntoResponse, LedgerError> {
    update_holding_price::execute(s.repo.as_ref(), update_holding_price::UpdateHoldingPriceCommand {
        owner_id:   s.user_id,
        account_id: AccountId::restore(account_id)?,
        ticker,
        new_price:  req.new_price,
        currency:   parse_currency(&req.currency)?,
    }).await?;
    Ok(StatusCode::OK)
}
