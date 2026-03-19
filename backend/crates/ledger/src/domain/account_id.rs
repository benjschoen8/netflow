use std::fmt;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use shared::domain::SharedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountId(Uuid);

impl fmt::Display for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AccountId {
    pub fn restore(id: Uuid) -> Result<Self, SharedError> {
        if id.is_nil() {
            return Err(SharedError::InvalidFormat(
                "[AccountId] contains illegal format (all zeros)"
            ));
        }
        Ok(Self(id))
    }

    pub fn create() -> Self {
        Self(Uuid::new_v4())
    }

    /// Return the inner UUID value.
    pub fn uuid(&self) -> uuid::Uuid {
        self.0
    }
}
