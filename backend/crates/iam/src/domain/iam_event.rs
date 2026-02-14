use chrono::{DateTime, Utc};
use crate::domain::role::Role;
use shared::domain::{EventId, UserId, Username, Email, DomainEvent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IamEvent {
    UserRegistered {
        user_id: UserId,
        username: Username,
        email: Email,
        role: Role,
    },
}

impl IamEvent {
    pub fn user_registered(user_id: UserId, username: Username, email:Email, role: Role) -> Self {
        Self::UserRegistered {
            user_id,
            username,
            email,
            role, 
        }
    }
}

impl DomainEvent for IamEvent {
    fn event_type(&self) -> &'static str {
        match self {
            IamEvent::UserRegistered { .. } => "iam.user_registered",
        }
    }
    
    fn event_version(&self) -> &'static str {
        match self {
            IamEvent::UserRegistered { .. } => "1.0",
        }
    }
}