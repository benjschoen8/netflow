use async_trait::async_trait;
use shared::domain::UserId;

use crate::domain::user_finances::UserFinances;
use crate::application::error::LedgerError;

/// Persistence port for the UserFinances aggregate.
/// Single responsibility: load and save aggregate state.
/// Event dispatching is handled separately by the use case layer.
#[async_trait]
pub trait UserFinancesRepository: Send + Sync {
    async fn load(&self, owner_id: UserId) -> Result<UserFinances, LedgerError>;

    /// Persist aggregate state. Takes an immutable reference — persistence
    /// has no reason to mutate the aggregate or pull its events.
    async fn save(&self, aggregate: &UserFinances) -> Result<(), LedgerError>;

    async fn exists(&self, owner_id: UserId) -> Result<bool, LedgerError>;
}
