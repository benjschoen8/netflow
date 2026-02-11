use uuid::Uuid;
use crate::shared::shared_error::SharedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventId(Uuid);

impl EventId {
    pub fn new() -> Self { Self(Uuid::new_v4()) }
    
    pub fn from(id: Uuid) -> Result<Self, SharedError> {
        if id.is_nil() {
            return Err(SharedError::InvalidFormat("[EventId] contains illegal format (all zeros)"));
        }
        Ok(Self(id))
    }
}
