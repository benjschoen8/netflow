use crate::application::event_metadata::EventMetadata;
#[derive(Debug, Clone)]
pub struct EventEnvelope<T> {
    pub metadata: EventMetadata,
    pub data: T,
}