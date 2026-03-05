use serde::Serialize;
use std::fmt;
use uuid::Uuid;

use crate::domain::shared_error::SharedError;
use crate::domain::aggregate_root_id::AggregateRootId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct UserId(Uuid);

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AggregateRootId for UserId {
    fn uuid(&self) -> Uuid {
        self.0
    }
}

impl UserId {
    pub fn restore(id: Uuid) -> Result<Self, SharedError> {
        if id.is_nil(){
            return Err(SharedError::InvalidFormat("[UserId] contains illegal format (all zeros)"));
        }
        Ok(Self(id))
    }
    
    pub fn create() -> Self { Self(Uuid::new_v4()) }
}