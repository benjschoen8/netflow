use async_trait::async_trait;
use crate::shared::domain::CorrelationId;

#[async_trait]
pub trait RepoStore<Aggregate>: Send + Sync {
    async fn save(&self, aggregate: Aggregate, correlation_id: CorrelationId) -> Result<(), AppError>;
}