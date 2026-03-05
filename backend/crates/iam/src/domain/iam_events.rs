use serde::Serialize;
use enum_dispatch::enum_dispatch;

use shared::domain::DomainEvent;
use crate::domain::events::user_registered::UserRegistered;

#[enum_dispatch(DomainEvent)]
#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum IamEvents {
    UserRegistered,
}