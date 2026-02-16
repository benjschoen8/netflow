use chrono::{DateTime,Utc};
use shared::domain::{EventId, UserId, Username, Email, Phone, AggregateRootId, AggregateRoot, DomainEvent};
use crate::domain::password_hash::PasswordHash;
use crate::domain::role::Role;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserRegistered {
    pub event_id: EventId,
    pub timestamp: DateTime<Utc>,
    pub user_id: UserId,
    pub username: Username,
    pub email: Email,
    pub phone: Option<Phone>,
    pub role: Role,
}

impl DomainEvent for UserRegistered {
    fn event_id(&self) -> EventId { self.event_id }
    fn event_type(&self) -> &'static str{ "iam.user_registered" }
    fn event_version(&self) -> &'static str{ "v1" }
    fn doamin(&self) -> &'static str{ "IAM" }
    fn service(&self) -> &'static str{ "IAM" }
}

impl UserRegistered {
    pub fn user_id(&self) -> UserId { self.user_id }
    pub fn username(&self) -> &Username { &self.username }
    pub fn email(&self) -> &Email { &self.email }
    pub fn phone(&self) -> &Option<Phone> { &self.phone }
    pub fn role(&self) -> Role { self.role }

    pub fn user_registered(user_id: UserId, username: Username, email:Email, phone:Option<Phone>, role: Role) -> Self {
        Self {
            event_id: EventId::new(),
            timestamp: Utc::now(),
            user_id,
            username,
            email,
            phone,
            role, 
        }
    }
}