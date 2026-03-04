use crate::domain::message::Message;
use crate::domain::correlation_id::CorrelationId;

pub struct MessageMapper;

impl MessageMapper {
    pub fn map<E: Clone>(
        events: &[E], 
        correlation_id: &CorrelationId
    ) -> Vec<Message<E>> {
        events
            .iter()
            .map(|event| Message::new(event.clone(), correlation_id.clone()))
            .collect()
    }
}