use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use crate::shared::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, DeriveValueType)]
pub struct Username(String);

impl Username {
    pub fn new(raw: String) -> Result<Self, SharedError> {
        if raw.is_empty() {
            return Err(SharedError::Empty(
                "[Username:raw] cannot be empty"
            ));
        }
        if raw.chars().any(|c| c.is_control()) {
            return Err(SharedError::InvalidFormat(
                "[Username:raw] contains illegal format (control characters)"
            ));
        }

        Ok(Self(raw))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}