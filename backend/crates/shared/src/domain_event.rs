use chrono::{DateTime, Utc};
use crate::event_id::EventId;
use crate::event_time_stamp::EventTimestamp;

pub trait DomainEvent: Send + Sync {
    fn event_id(&self) -> EventId;

    fn occurred_on(&self) -> EventTimestamp;

    fn event_type(&self) -> &str;
    
    fn event_version(&self) -> &str;
}