use std::fmt;
use uuid::Uuid;

use crate::domain::shared_error::SharedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CorrelationId(Uuid);

impl fmt::Display for CorrelationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CorrelationId {
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from(incoming: Uuid) -> Result<Self, SharedError> {
        if incoming.is_nil() {
            return Err(SharedError::InvalidFormat("CorrelationId cannot be nil"));
        }
        Ok(Self(incoming))
    }
}