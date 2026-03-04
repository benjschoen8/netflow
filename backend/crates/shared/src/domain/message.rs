use crate::domain::message_id::MessageId;
use crate::domain::correlation_id::CorrelationId;

#[derive(Debug, Clone)]
pub struct Message<T> {
    pub message_id: MessageId,
    pub correlation_id: CorrelationId,
    pub data: T,
}

impl<T> Message<T> {
    pub fn new(data: T , correlation_id: CorrelationId) -> Self {
        Self {
            message_id: MessageId::new(),
            correlation_id,
            data,
        }
    }
}