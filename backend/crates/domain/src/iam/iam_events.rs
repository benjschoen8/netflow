use chrono::{DateTime, Utc};

use crate::shared::user_id::UserId;
use crate::iam::role::Role;
use crate::shared::domain_event::DomainEvent;
use crate::shared::event_id::EventId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IamEvent {
    UserRegistered {
        event_id: EventId,
        occurred_at: DateTime<Utc>,
        user_id: UserId,
        role: Role,
    },
}

impl Event for IamEvent {
    fn event_id(&self) -> EventId {
        return self.event_id
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        return self.occurred_at;
    }
}
        