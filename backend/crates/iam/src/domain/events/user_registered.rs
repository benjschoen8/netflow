use serde::Serialize;

use shared::domain::{EventId, Timestamp, UserId, Username, Email, Phone, AggregateRootId, AggregateRoot, DomainEvent};
use crate::domain::password_hash::PasswordHash;
use crate::domain::role::Role;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct UserRegistered {
    pub event_id: EventId,
    pub occured_on: Timestamp,
    pub user_id: UserId,
    pub username: Username,
    pub email: Email,
    pub phone: Option<Phone>,
    pub role: Role,
}

impl DomainEvent for UserRegistered {
    fn event_id(&self) -> EventId { self.event_id }
    fn occurred_on(&self) -> Timestamp { self.occured_on }
    fn event_type(&self) -> &'static str{ "iam.user_registered" }
    fn event_version(&self) -> &'static str{ "v1" }
    fn domain(&self) -> &'static str{ "IAM" }
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
            occured_on: Timestamp::now(),
            user_id,
            username,
            email,
            phone,
            role, 
        }
    }
}