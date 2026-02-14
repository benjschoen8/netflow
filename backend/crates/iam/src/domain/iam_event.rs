use chrono::{DateTime, Utc};
use crate:shared::user_id::UserId;
use crate::role::Role;
use crate:shared::event_id::EventId;
use crate:shared::domain_event::DomainEvent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IamEvent {
    UserRegistered {
        event_id: EventId,
        occurred_on: DateTime<Utc>,
        user_id: UserId,
        role: Role,
    },
}

impl IamEvent {
    pub fn user_registered(user_id: UserId, role: Role) -> Self {
        Self::UserRegistered {
            event_id: EventId::new(),
            occurred_on: Utc::now(),
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

    fn occurred_on(&self) -> DateTime<Utc> {
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