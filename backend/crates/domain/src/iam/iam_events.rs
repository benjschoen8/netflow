use crate::shared::user_id::UserId;
use crate::iam::role::Role;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IamEvent {
    UserRegistered {
        user_id: UserId,
        role: Role,
    },
}