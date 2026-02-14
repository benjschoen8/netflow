use uuid::Uuid;
use crate::domain::domain_event::DomainEvent;
use crate::domain::correlation_id::CorrelationId;
use crate::application::event_bus::EventBus;
use crate::infrastructure::envelope_factory::EnvelopeFactory;

pub struct EventBridge {
    factory: EnvelopeFactory,
}

impl EventBus for EventBridge {
    fn publish<T: DomainEvent>(&self, event: T, correlation_id: CorrelationId) {
        let envelope = self.factory.wrap(event, correlation_id);
    }
}