use crate::domain::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountNumber(String);

impl AccountNumber {
    pub fn new(number: String) -> Result<Self, SharedError> {
        let trimmed = val.trim().to_string();
        if trimmed.is_empty() {
            return Err(SharedError::Empty("[AccountNumber] cannot be empty"));
        }
        if trimmed.chars().any(|c| c.is_control()) {
            return Err(SharedError::InvalidFormat("[AccountNumber] contains illegal format (control characters)"));
        }
        Ok(Self(trimmed))
    }
}
