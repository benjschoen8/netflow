use serde::Serialize;
use crate::domain::event_id::EventId;
use crate::domain::timestamp::Timestamp;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EventMetadata {
    event_id: EventId,
    occurred_on: Timestamp,
}

impl EventMetadata {
    pub fn event_id(&self) -> EventId {
        self.event_id
    }

    pub fn occurred_on(&self) -> Timestamp {
        self.occurred_on()
    }

    pub fn now() -> Self {
        Self {
            event_id: EventId::new(),
            occurred_on: Timestamp::now(),
        }
    }
}