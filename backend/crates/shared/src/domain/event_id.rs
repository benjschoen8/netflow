use uuid::Uuid;
use crate::domain::shared_error::SharedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventId(Uuid);

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
