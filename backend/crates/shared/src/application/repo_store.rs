use async_trait::async_trait;
use crate::domain::shared_error::SharedError;
use crate::domain::message::Message;
use crate::domain::domain_event::DomainEvent;

#[async_trait]
pub trait RepoStore<Aggregate, Event: DomainEvent>: Send + Sync {
    async fn save(&self, aggregate: Aggregate, messages: &Vec<Message<Event>>) -> Result<(), SharedError>;
}