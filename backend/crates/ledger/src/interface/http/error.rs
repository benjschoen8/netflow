use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;
use crate::application::error::LedgerError;

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for LedgerError {
    fn into_response(self) -> Response {
        let status = match &self {
            LedgerError::FinancesNotFound          => StatusCode::NOT_FOUND,
            LedgerError::AccountNotFound(_)        => StatusCode::NOT_FOUND,
            LedgerError::WrongAccountType(_)       => StatusCode::UNPROCESSABLE_ENTITY,
            LedgerError::Validation(_)             => StatusCode::BAD_REQUEST,
            LedgerError::Domain(_)                 => StatusCode::UNPROCESSABLE_ENTITY,
            LedgerError::Repository(_)             => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(ErrorBody { error: self.to_string() })).into_response()
    }
}
