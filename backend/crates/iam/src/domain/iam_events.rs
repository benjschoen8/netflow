use enum_dispatch::enum_dispatch;

use shared::domain::DomainEvent;
use crate::domain::events::user_registered::UserRegistered;

#[enum_dispatch(DomainEvent)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IamEvents {
    UserRegistered,
}