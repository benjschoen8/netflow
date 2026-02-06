use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::shared::shared_error::SharedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountId(Uuid);

impl AccountId {
    pub fn new() -> Self { Self(Uuid::new_v4()) }
    
    pub fn from_uuid(id: Uuid) -> Result<Self, SharedError> {
        if id.is_nil() {
            return Err(SharedError::InvalidFormat("[AccountId:id] contains illegal format (all zeros)"));
        }
        Ok(Self(id))
    }
}
