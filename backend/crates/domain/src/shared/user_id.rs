use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::shared::shared_error::SharedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountId(Uuid);

impl AccountId {
    pub fn from(id: Uuid) -> Result<Self, SharedError> {
        if id.is_nil() {
            return Err(SharedError::InvalidFormat("[AccountId] contains illegal format (all zeros)"));
        }
        Ok(Self(id))
    }
    
    pub fn new() -> Self { Self(Uuid::new_v4()) }
}