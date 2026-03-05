use crate::domain::message::Message;
use crate::domain::correlation_id::CorrelationId;
use crate::domain::domain_event::DomainEvent;

pub struct MessageMapper;

impl MessageMapper {
    pub fn map<E: DomainEvent + Clone>(
        events: Vec<E>, 
        correlation_id: &CorrelationId
    ) -> Vec<Message<E>> {
        events
            .iter()
            .map(|event| Message::new(event.clone(), correlation_id.clone()))
            .collect()
    }
}