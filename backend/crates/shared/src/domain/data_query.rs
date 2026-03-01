use async_trait::async_trait;
use crate::shared::domain::CorrelationId;

#[async_trait]
pub trait DataQuery<aggregate>: Send + Sync {
    async fn by_uuid(&self) -> Result<aggregate, shared_error>;
}