use crate::domain::message_id::MessageId;
use crate::domain::correlation_id::CorrelationId;
use crate::domain::event_id::EventId;
use crate::domain::timestamp::Timestamp;
use crate::domain::domain_event::DomainEvent;

#[derive(Debug, Clone)]
pub struct Message<T: DomainEvent> {
    message_id: MessageId,
    correlation_id: CorrelationId,
    data: T,
}

impl<T: DomainEvent> Message<T> {
    pub fn new(data: T, correlation_id: CorrelationId) -> Self {
        Self {
            message_id: MessageId::new(),
            correlation_id,
            data,
        }
    }

    pub fn message_id(&self) -> MessageId { self.message_id }
    pub fn correlation_id(&self) -> CorrelationId { self.correlation_id }
    pub fn data(&self) -> &T { &self.data }

    pub fn event_id(&self) -> EventId { self.data.event_id() }
    pub fn occurred_on(&self) -> Timestamp { self.data.occurred_on() }  // fixed typo
    pub fn event_type(&self) -> &'static str { self.data.event_type() }
    pub fn event_version(&self) -> &'static str { self.data.event_version() }
    pub fn domain(&self) -> &'static str { self.data.domain() }
    pub fn service(&self) -> &'static str { self.data.service() }
}