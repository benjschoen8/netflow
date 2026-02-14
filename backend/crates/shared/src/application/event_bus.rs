use crate::domain::correlation_id::CorrelationId;
use crate::domain::domain_event::DomainEvent;

pub trait EventBus {
    fn publish<T: DomainEvent>(&self, event: T, correlation_id: CorrelationId);
}