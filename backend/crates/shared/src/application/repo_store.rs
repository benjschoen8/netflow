use async_trait::async_trait;
use crate::domain::shared_error::SharedError;
use crate::domain::correlation_id::CorrelationId;

#[async_trait]
pub trait RepoStore<Aggregate>: Send + Sync {
    async fn save(&self, aggregate: Aggregate, correlation_id: CorrelationId) -> Result<(), SharedError>;
}