use serde::{Deserialize, Serialize};
use crate::shared::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountName(String);

impl AccountName {
    pub fn new(name: String) -> Result<Self, SharedError> {
        if name.is_empty() {
            return Err(SharedError::Empty("[AccountName:name] cannot be empty"));
        }
        if name.chars().any(|c| c.is_control()) {
            return Err(SharedError::InvalidFormat("[AccountName:name] contains illegal format (control characters)"));
        }
        Ok(Self(name))
    }
}