use serde::{Deserialize, Serialize};
use crate::shared::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountNumber(String);

impl AccountNumber {
    pub fn new(number: String) -> Result<Self, SharedError> {
        if number.is_empty() {
            return Err(SharedError::Empty("[AccountNumber:number] cannot be empty"));
        }
        if number.chars().any(|c| c.is_control()) {
            return Err(SharedError::InvalidFormat("[AccountNumber:number] contains illegal format (control characters)"));
        }
        Ok(Self(number))
    }
}
