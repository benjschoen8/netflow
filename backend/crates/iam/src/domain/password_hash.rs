use std::fmt;
use crate:shared::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(val: String) -> Result<Self, SharedError> {
        if val.is_empty() {
            return Err(SharedError::Empty("[PasswordHash:val] cannot be empty"));
        }
        if val.chars().any(|c| c.is_control()) {
            return Err(SharedError::InvalidFormat("[PasswordHash:val] contains illegal format (control characters)"));
        }
        Ok(Self(val))
    }
}