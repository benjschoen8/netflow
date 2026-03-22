use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::application::error::LedgerError;
use crate::domain::account_id::AccountId;
use crate::interface::http::app_state::AppState;

/// GET /accounts/:id/entries
pub async fn list(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, LedgerError> {
    let account_id = AccountId::restore(id)?;
    let entries    = s.entry_repo.list_for_account(account_id).await?;
    Ok(Json(entries))
}

/// PATCH /entries/:entry_id
#[derive(Deserialize)]
pub struct AnnotationRequest {
    pub label:       Option<String>,
    pub description: Option<String>,
}

pub async fn update_annotation(
    State(s): State<AppState>,
    Path(entry_id): Path<Uuid>,
    Json(req): Json<AnnotationRequest>,
) -> Result<impl IntoResponse, LedgerError> {
    // Verify entry exists before updating
    s.entry_repo
        .find(entry_id)
        .await?
        .ok_or(LedgerError::Validation(format!("Entry {} not found", entry_id)))?;

    s.entry_repo
        .update_annotation(entry_id, req.label, req.description)
        .await?;

    Ok(StatusCode::OK)
}
