use async_trait::async_trait;
use crate::domain::shared_error::SharedError;
use crate::domain::message::Message;

#[async_trait]
pub trait RepoStore<Aggregate, Event>: Send + Sync {
    async fn save(&self, aggregate: Aggregate, messages:Vec<Message<Event>>) -> Result<(), SharedError>;
}