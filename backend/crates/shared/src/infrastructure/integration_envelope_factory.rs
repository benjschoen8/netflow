use chrono::Utc;
use uuid::Uuid;
use crate::domain::event_id::EventId;
use crate::domain::correlation_id::CorrelationId;
use crate::domain::domain_event::DomainEvent;
use crate::infrastructure::integration_event_metadata::IntegrationEventMetadata;
use crate::application::event_envelope::EventEnvelope;

pub struct IntegrationEnvelopeFactory {
    service_name: &'static str,
    domain_name: &'static str,
}

impl IntegrationEnvelopeFactory {
    pub fn wrap<T: DomainEvent>(&self, event: T, correlation_id: CorrelationId) -> EventEnvelope<T> {
        EventEnvelope {
            metadata: IntegrationEventMetadata {
                id: &EventId::new(),
                version: event.event_version(),
                timestamp: Utc::now(),
                domain: self.domain_name.clone(),
                service: self.service_name.clone(),
                event_type: event.event_type(),
                correlation_id,
            },
            data: event,
        }
    }
}