use thiserror::Error;
use shared::domain::SharedError;
use uuid::Uuid;

/// All errors that can escape the Application layer.
///
/// Value-object types (AccountId etc.) are NOT used here because this error
/// crosses layer boundaries. We carry the raw Uuid so the interface layer
/// has no dependency on domain types just to read an error.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum LedgerError {
    // ── Domain rule violations ─────────────────────────────────────────────
    #[error("Domain error: {0}")]
    Domain(#[from] SharedError),

    // ── Not-found variants ─────────────────────────────────────────────────
    #[error("No finances record found for this user. Run `netflow init` first.")]
    FinancesNotFound,

    #[error("Account {0} does not exist.")]
    AccountNotFound(Uuid),

    // ── Wrong type / guard errors ──────────────────────────────────────────
    #[error("Account {0} is not the right type for this operation.")]
    WrongAccountType(Uuid),

    // ── Persistence errors ─────────────────────────────────────────────────
    #[error("Storage error: {0}")]
    Repository(String),

    // ── Validation errors (from CLI / HTTP command parsing) ─────────────────
    #[error("Invalid input: {0}")]
    Validation(String),
}
