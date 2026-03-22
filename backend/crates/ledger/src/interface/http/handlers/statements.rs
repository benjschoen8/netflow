//! Statement endpoints for credit card billing cycles.

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
use crate::domain::statement::Statement;
use crate::interface::http::app_state::AppState;

// ── Response shape ────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct StatementResponse {
    pub id:                String,
    pub account_id:        String,
    pub cycle_start:       String,   // YYYY-MM-DD
    pub cycle_end:         String,   // YYYY-MM-DD
    pub statement_balance: String,
    pub minimum_payment:   Option<String>,
    pub total_charged:     String,
    pub total_paid:        String,
    pub remaining:         String,
    pub is_settled:        bool,
    pub created_at:        String,
}

impl From<Statement> for StatementResponse {
    fn from(s: Statement) -> Self {
        Self {
            id:                s.id.to_string(),
            account_id:        s.account_id.uuid().to_string(),
            cycle_start:       s.cycle_start.to_string(),
            cycle_end:         s.cycle_end.to_string(),
            statement_balance: s.statement_balance.to_string(),
            minimum_payment:   s.minimum_payment.map(|d| d.to_string()),
            total_charged:     s.total_charged.to_string(),
            total_paid:        s.total_paid.to_string(),
            remaining:         s.remaining().to_string(),
            is_settled:        s.is_settled,
            created_at:        s.created_at.to_rfc3339(),
        }
    }
}

/// `GET /accounts/:id/statements`
/// All statements for this credit card, newest first.
pub async fn list(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, LedgerError> {
    let account_id = AccountId::restore(id)?;
    let statements = s.statement_repo.list_for_account(account_id).await?;
    let response: Vec<StatementResponse> = statements.into_iter().map(Into::into).collect();
    Ok(Json(response))
}

/// `GET /accounts/:id/statements/:stmt_id/entries`
/// All ledger entries that fall within a statement's billing cycle.
pub async fn statement_entries(
    State(s): State<AppState>,
    Path((account_id, stmt_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, LedgerError> {
    let aid  = AccountId::restore(account_id)?;
    let stmt = s.statement_repo
        .find(stmt_id)
        .await?
        .ok_or_else(|| LedgerError::Validation(format!("Statement {} not found", stmt_id)))?;

    // Verify this statement belongs to this account
    if stmt.account_id != aid {
        return Err(LedgerError::Validation(
            "Statement does not belong to this account".to_string(),
        ));
    }

    // Fetch all entries for the account then filter to the cycle window
    let all_entries = s.entry_repo.list_for_account(aid).await?;
    let cycle_start = stmt.cycle_start.and_hms_opt(0, 0, 0).unwrap()
        .and_utc();
    let cycle_end   = stmt.cycle_end.and_hms_opt(23, 59, 59).unwrap()
        .and_utc();

    let filtered: Vec<_> = all_entries
        .into_iter()
        .filter(|e| e.occurred_at >= cycle_start && e.occurred_at <= cycle_end)
        .collect();

    Ok(Json(filtered))
}
