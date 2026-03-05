use crate::domain::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountNumber(String);

impl fmt::Display for AccountNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
