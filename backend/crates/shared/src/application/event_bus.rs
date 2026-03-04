use crate::domain::shared_error::SharedError;
use crate::domain::correlation_id::CorrelationId;
use crate::domain::domain_event::DomainEvent;

pub trait EventBus {
    type Message;
    async fn publish(&self, messages: Vec<Self::Message>) -> Result<(), SharedError>;
}