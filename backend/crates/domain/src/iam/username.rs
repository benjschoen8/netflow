use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use crate::shared::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, DeriveValueType)]
pub struct Username(String);

impl Username {
    pub fn new(val: String) -> Result<Self, SharedError> {
        if val.is_empty() {
            return Err(SharedError::Empty("[Username:val] cannot be empty"));
        }
        if val.chars().any(|c| c.is_control()) {
            return Err(SharedError::InvalidFormat("[Username:val] contains illegal format (control characters)"));
        }
        Ok(Self(val))
    }
}