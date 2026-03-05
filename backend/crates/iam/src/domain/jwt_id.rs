use std::fmt;
use uuid::Uuid;

use shared::domain::SharedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JwtId(Uuid);

impl fmt::Display for JwtId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl JwtId {
    pub fn new() -> Self { Self(Uuid::new_v4()) }
    
    pub fn from_uuid(id: Uuid) -> Result<Self, SharedError> {
        if id.is_nil() {
            return Err(SharedError::InvalidFormat("[JwtId] contains illegal format (all zeros)"));
        }
        Ok(Self(id))
    }
}
