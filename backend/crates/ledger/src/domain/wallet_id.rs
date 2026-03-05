use std::fmt;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use shared::domain::SharedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletId(Uuid);

impl WalletId {
    pub fn create() -> Self { Self(Uuid::new_v4()) }

    pub fn restore(id: Uuid) -> Result<Self, SharedError> {
        if id.is_nil() {
            return Err(SharedError::InvalidFormat(
                "[WalletId] contains illegal format (all zeros)"
            ));
        }
        Ok(Self(id))
    }
}

impl fmt::Display for WalletId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
