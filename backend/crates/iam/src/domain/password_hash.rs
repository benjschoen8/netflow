use std::fmt;
use shared::domain::SharedError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordHash(String);

impl fmt::Display for PasswordHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PasswordHash {
    pub fn new(val: String) -> Result<Self, SharedError> {
        if val.is_empty() {
            return Err(SharedError::Empty("[PasswordHash] cannot be empty"));
        }
        if val.chars().any(|c| c.is_control()) {
            return Err(SharedError::InvalidFormat("[PasswordHash] contains illegal format (control characters)"));
        }
        Ok(Self(val))
    }
}