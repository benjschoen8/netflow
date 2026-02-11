use crate::shared::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bank(String);

impl Bank {
    pub fn new(val: String) -> Result<Self, SharedError> {
        if val.is_empty() {
            return Err(SharedError::Empty("[Bank] cannot be empty"));
        }
        if val.chars().any(|c| c.is_control()) {
            return Err(SharedError::InvalidFormat("[Bank] contains illegal format (control characters)"));
        }
        Ok(Self(val))
    }
}