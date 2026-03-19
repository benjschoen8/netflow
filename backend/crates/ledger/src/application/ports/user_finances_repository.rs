use async_trait::async_trait;
use shared::domain::UserId;

use crate::domain::user_finances::UserFinances;
use crate::application::error::LedgerError;

/// The only persistence port the Application layer knows about.
///
/// The Infrastructure layer provides the concrete SQLite implementation;
/// the Application layer depends only on this trait (hexagonal port).
#[async_trait]
pub trait UserFinancesRepository: Send + Sync {
    /// Load the aggregate for a user.
    /// Returns `LedgerError::FinancesNotFound` if none exists yet.
    async fn load(&self, owner_id: UserId) -> Result<UserFinances, LedgerError>;

    /// Persist the full aggregate state.
    /// This is an upsert — creates on first save, updates thereafter.
    async fn save(&self, aggregate: &UserFinances) -> Result<(), LedgerError>;

    /// Check existence without loading the full aggregate.
    async fn exists(&self, owner_id: UserId) -> Result<bool, LedgerError>;
}
