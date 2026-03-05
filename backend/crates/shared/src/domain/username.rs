use serde::Serialize;
use std::fmt;

use crate::domain::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Username(String);

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Username {
    pub fn new(val: String) -> Result<Self, SharedError> {
        let trimmed = val.trim().to_string();
        if trimmed.is_empty() {
            return Err(SharedError::Empty("[Username] cannot be empty"));
        }
        if trimmed.chars().any(|c| c.is_control()) {
            return Err(SharedError::InvalidFormat("[Username] contains illegal format (control characters)"));
        }
        Ok(Self(trimmed))
    }

    pub fn as_str(&self) -> &str { self.0.as_str() }
}