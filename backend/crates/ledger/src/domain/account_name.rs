use shared::domain::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountName(String);

impl fmt::Display for AccountName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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