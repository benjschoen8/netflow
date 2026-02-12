use chrono::{DateTime, Utc};
use crate::shared::event_id::EventId;

pub trait DomainEvent: std::fmt::Debug + Send + Sync {
    fn event_id(&self) -> EventId;
    fn occurred_at(&self) -> DateTime<Utc>;
    fn event_type(&self) -> &'static str;
}