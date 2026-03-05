use std::fmt;
use serde::{Serialize, Deserialize};

use shared::domain::SharedError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountNumber(String);

impl fmt::Display for AccountNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AccountNumber {
    pub fn new(number: String) -> Result<Self, SharedError> {
        let trimmed = number.trim().to_string(); // fixed: was `val`, now `number`
        if trimmed.is_empty() {
            return Err(SharedError::Empty("[AccountNumber] cannot be empty"));
        }
        if trimmed.chars().any(|c| c.is_control()) {
            return Err(SharedError::InvalidFormat(
                "[AccountNumber] contains illegal format (control characters)"
            ));
        }
        Ok(Self(trimmed))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
