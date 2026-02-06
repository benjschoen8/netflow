use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use crate::shared::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Bank{
    id: String,
    name: String,
}

impl Bank {
    pub fn new(id: String, name: String) -> Result<Self, SharedError> {
        if id.is_empty() {
            return Err(SharedError::Empty("[Bank:id] cannot be empty"));
        }
        if id.chars().any(|c| c.is_control()) {
            return Err(SharedError::InvalidFormat("[Bank:id] contains illegal format (control characters)"));
        }
        if name.is_empty() {
            return Err(SharedError::Empty("[Bank:name] cannot be empty"));
        }
        if name.chars().any(|c| c.is_control()) {
            return Err(SharedError::InvalidFormat("[Bank:name] contains illegal format (control characters)"));
        }
        Ok(Self { id, name })
    }
}