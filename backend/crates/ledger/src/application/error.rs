use thiserror::Error;
use shared::domain::SharedError;
use crate::domain::account_id::AccountId;

/// All errors that can escape the Application layer.
///
/// The outer Interface layer maps these into user-facing CLI messages.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum LedgerError {
    // ── Domain rule violations ─────────────────────────────────────────────
    #[error("Domain error: {0}")]
    Domain(#[from] SharedError),

    // ── Not-found variants ─────────────────────────────────────────────────
    #[error("No finances record found for this user. Run `netflow init` first.")]
    FinancesNotFound,

    #[error("Account '{0}' does not exist.")]
    AccountNotFound(AccountId),

    // ── Wrong type / guard errors ──────────────────────────────────────────
    #[error("Account '{0}' is not the right type for this operation.")]
    WrongAccountType(AccountId),

    // ── Persistence errors ─────────────────────────────────────────────────
    #[error("Storage error: {0}")]
    Repository(String),

    // ── Validation errors (from CLI / command parsing) ─────────────────────
    #[error("Invalid input: {0}")]
    Validation(String),
}
