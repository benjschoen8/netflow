use chrono::{DateTime, Utc};
use crate::shared::event_id::EventId;

pub trait DomainEvent: Send + Sync {
    fn event_id(&self) -> EventId;

    fn occurred_on(&self) -> DateTime<Utc>;

    fn event_type(&self) -> &str;
    
    fn event_version(&self) -> &str;
}