use crate:shared::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenHash(String);

impl TokenHash {
    pub fn new(val: String) -> Result<Self, SharedError> {
        if val.is_empty() {
            return Err(SharedError::Empty("[TokenHash] cannot be empty"));
        }
        if val.chars().any(|c| c.is_control()) {
            return Err(SharedError::InvalidFormat("[TokenHash] contains illegal format (control characters)"));
        }
        Ok(Self(val))
    }
}

impl PartialEq<&str> for TokenHash {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}