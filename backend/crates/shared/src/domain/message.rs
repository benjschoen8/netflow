use crate::domain::message_id::MessageId;
use crate::domain::correlation_id::CorrelationId;

#[derive(Debug, Clone)]
pub struct Message<T> {
    message_id: MessageId,
    correlation_id: CorrelationId,
    data: T,
}

impl<T> Message<T> {
    pub fn new(data: T , correlation_id: CorrelationId) -> Self {
        Self {
            message_id: MessageId::new(),
            correlation_id,
            data,
        }
    }

    pub fn message_id(&self) -> MessageId {
        self.message_id
    }

    pub fn correlation_id(&self) -> CorrelationId {
        self.correlation_id
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    fn event_id(&self) -> EventId { self.data.event_id() }
    fn occured_on(&self) -> Timestamp { self.data.occured_on() }
    fn event_type(&self) -> &'static str{ self.data.event_type() }
    fn event_version(&self) -> &'static str{ self.data.event_version() }
    fn domain(&self) -> &'static str{ self.data.domain() }
    fn service(&self) -> &'static str{ self.data.service() }
}