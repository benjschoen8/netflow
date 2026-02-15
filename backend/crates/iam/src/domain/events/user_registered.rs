use shared::domain::{UserId, Username, Email, AggregateRootId, AggregateRoot, DomainEvent};
use crate::domain::role::Role;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserRegistered {
    pub user_id: UserId,
    pub username: Username,
    pub email: Email,
    pub role: Role,
}

impl DomainEvent for UserRegistered {
    fn aggregate_root_id(&self) -> String{ self.user_id.to_string() }
    fn event_type(&self) -> &'static str{ "iam.user_registered" }
    fn event_version(&self) -> &'static str{ "v1" }
}

impl UserRegistered {
    pub fn user_registered(user_id: UserId, username: Username, email:Email, role: Role) -> Self {
        Self {
            user_id,
            username,
            email,
            role, 
        }
    }
}