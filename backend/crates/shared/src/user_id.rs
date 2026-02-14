use uuid::Uuid;
use crate::shared_error::SharedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserId(Uuid);

impl UserId {
    pub fn restore(id: Uuid) -> Result<Self, SharedError> {
        if id.is_nil() {
            return Err(SharedError::InvalidFormat("[UserId] contains illegal format (all zeros)"));
        }
        Ok(Self(id))
    }
    
    pub fn create() -> Self { Self(Uuid::new_v4()) }
}