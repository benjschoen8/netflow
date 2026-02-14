use chrono::{DateTime, Utc};
use shared::UserId;
use crate::domain::role::Role;
use shared::EventId;
use shared::DomainEvent;
use shared::EventTimestamp;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IamEvent {
    UserRegistered {
        event_id: EventId,
        occurred_on: EventTimestamp,
        user_id: UserId,
        role: Role,
    },
}

impl IamEvent {
    pub fn user_registered(user_id: UserId, role: Role) -> Self {
        Self::UserRegistered {
            event_id: EventId::new(),
            occurred_on: EventTimestamp::now(),
            user_id,
            role,
        }
    }
}

impl DomainEvent for IamEvent {
    fn event_id(&self) -> EventId {
        match self {
            IamEvent::UserRegistered { event_id, .. } => *event_id,
        }
    }

    fn occurred_on(&self) -> EventTimestamp {
        match self {
            IamEvent::UserRegistered { occurred_on, .. } => *occurred_on,
        }
    }

    fn event_type(&self) -> &str {
        match self {
            IamEvent::UserRegistered { .. } => "iam.user_registered",
        }
    }
    
    fn event_version(&self) -> &str {
        match self {
            IamEvent::UserRegistered { .. } => "1.0",
        }
    }
}