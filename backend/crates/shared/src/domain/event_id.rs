use serde::Serialize;
use std::fmt;
use uuid::Uuid;

use crate::domain::shared_error::SharedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct EventId(Uuid);

impl fmt::Display for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl EventId {
    pub fn uuid(&self) -> Uuid { self.0 }

    pub fn new() -> Self { Self(Uuid::new_v4()) }
    
    pub fn from(id: Uuid) -> Result<Self, SharedError> {
        if id.is_nil() {
            return Err(SharedError::InvalidFormat("[EventId] contains illegal format (all zeros)"));
        }
        Ok(Self(id))
    }
}
