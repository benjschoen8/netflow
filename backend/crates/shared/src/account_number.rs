use crate::shared::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountNumber(String);

impl AccountNumber {
    pub fn new(number: String) -> Result<Self, SharedError> {
        if number.is_empty() {
            return Err(SharedError::Empty("[AccountNumber] cannot be empty"));
        }
        if number.chars().any(|c| c.is_control()) {
            return Err(SharedError::InvalidFormat("[AccountNumber] contains illegal format (control characters)"));
        }
        Ok(Self(number))
    }
}
