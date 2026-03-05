use async_trait::async_trait;

use crate::domain::shared_error::SharedError;

#[async_trait]
pub trait EventBus: Send + Sync {
    type Message;
    async fn publish(
        &self, 
        messages: Vec<Self::Message>
    ) -> Result<(), SharedError>;
}