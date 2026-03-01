use async_trait::async_trait;
use crate::shared::domain::CorrelationId;

#[async_trait::async_trait]
pub trait Outbox: Send + Sync {
    async fn fetch_pending_batch(&self, size: i32) -> Result<Vec<OutboxEvent>, InfrastructureError>;
    async fn mark_as_published(&self, ids: &[Uuid]) -> Result<(), InfrastructureError>;
}