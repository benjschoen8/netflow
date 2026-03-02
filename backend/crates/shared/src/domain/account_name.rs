use crate::domain::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountName(String);

impl AccountName {
    pub fn new(val: String) -> Result<Self, SharedError> {
        let trimmed = val.trim().to_string();
        if trimmed.is_empty() {
            return Err(SharedError::Empty("[AccountName] cannot be empty"));
        }
        if trimmed.chars().any(|c| c.is_control()) {
            return Err(SharedError::InvalidFormat("[AccountName] contains illegal format (control characters)"));
        }
        Ok(Self(trimmed))
    }
}