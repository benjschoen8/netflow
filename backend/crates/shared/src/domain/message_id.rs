use uuid::Uuid;
use std::fmt;
use crate::domain::shared_error::SharedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageId(Uuid);

impl MessageId {
    pub fn uuid(&self) -> Uuid { self.0 }

    pub fn new() -> Self { Self(Uuid::new_v4()) }
    
    pub fn from(id: Uuid) -> Result<Self, SharedError> {
        if id.is_nil() {
            return Err(SharedError::InvalidFormat("[EventId] contains illegal format (all zeros)"));
        }
        Ok(Self(id))
    }
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

