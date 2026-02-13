use chrono::{DateTime, Utc};

use crate::shared::event_id::EventId;

pub trait Event {
    fn event_id(&self) -> &EventId;
    fn occurred_at(&self) -> DateTime<Utc>;
}