use uuid::Uuid;
use crate::shared::shared_error::SharedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JwtId(Uuid);

impl JwtId {
    pub fn new() -> Self { Self(Uuid::new_v4()) }
    
    pub fn from_uuid(id: Uuid) -> Result<Self, SharedError> {
        if id.is_nil() {
            return Err(SharedError::InvalidFormat("[JwtId] contains illegal format (all zeros)"));
        }
        Ok(Self(id))
    }
}
