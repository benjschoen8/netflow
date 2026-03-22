//! Atomic write port for the ledger bounded context.
//!
//! Replaces the previous pattern of calling `repo.save()` + `entry_repo.save()`
//! sequentially without a transaction. The infrastructure implementation wraps
//! all writes in a single `BEGIN IMMEDIATE / COMMIT` so that aggregate state
//! and its audit entries are always consistent.
//!
//! Query use cases continue to use `UserFinancesRepository` for `load()`.
//! This port is only for mutation use cases.

use async_trait::async_trait;
use rust_decimal::Decimal;
use shared::domain::UserId;
use uuid::Uuid;

use crate::application::error::LedgerError;
use crate::domain::ledger_entry::LedgerEntry;
use crate::domain::statement::Statement;
use crate::domain::user_finances::UserFinances;

/// Everything that must be written atomically in a single mutation.
/// Fields are additive — set only what the current use case needs.
pub struct WriteOperation<'a> {
    /// The mutated aggregate whose state must be persisted.
    pub aggregate:        &'a UserFinances,

    /// Ledger entries to insert (can be zero, one, or many).
    pub entries:          Vec<LedgerEntry>,

    /// A newly-created Statement record (close_statement only).
    pub new_statement:    Option<Statement>,

    /// Apply a payment to an existing open statement (make_payment on credit card).
    /// `(statement_id, payment_amount)` — increments `total_paid`, recalculates `is_settled`.
    pub credit_statement: Option<(Uuid, Decimal)>,
}

impl<'a> WriteOperation<'a> {
    /// Convenience constructor for the common case: just state + entries.
    pub fn new(aggregate: &'a UserFinances, entries: Vec<LedgerEntry>) -> Self {
        Self {
            aggregate,
            entries,
            new_statement:    None,
            credit_statement: None,
        }
    }
}

/// Atomic write unit for the ledger.
///
/// Mutation use cases receive `&dyn LedgerUnitOfWork` and call:
/// 1. `load()` — read current aggregate state
/// 2. mutate the aggregate in memory
/// 3. `commit(WriteOperation { ... })` — persist everything atomically
///
/// The infrastructure implementation runs all writes inside a single
/// `BEGIN IMMEDIATE / COMMIT` SQLite transaction.
#[async_trait]
pub trait LedgerUnitOfWork: Send + Sync {
    /// Load the aggregate. Same semantics as `UserFinancesRepository::load`.
    async fn load(&self, owner_id: UserId) -> Result<UserFinances, LedgerError>;

    /// Check whether a finances record exists without loading it.
    async fn exists(&self, owner_id: UserId) -> Result<bool, LedgerError>;

    /// Atomically persist all writes in `op` inside a single DB transaction.
    /// If any write fails the entire transaction is rolled back.
    async fn commit(&self, op: WriteOperation<'_>) -> Result<(), LedgerError>;
}
