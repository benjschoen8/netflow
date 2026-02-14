use chrono::{DateTime, Utc};
use crate::domain::event_id::EventId;
use crate::domain::correlation_id::CorrelationId;

#[derive(Debug, Clone)]
pub struct EventMetadata {
    pub id: EventId,
    pub version: &'static str,
    pub timestamp: DateTime<Utc>,
    pub domain: &'static str,
    pub service: &'static str,
    pub event_type: &'static str,
    pub correlation_id: CorrelationId,
}