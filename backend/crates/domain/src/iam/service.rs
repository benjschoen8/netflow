use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Service(String);

impl Service {
    pub fn new(val: String) -> Result<Self, SharedError> {
        if val.is_empty() {
            return Err(SharedError::Empty("[Service] cannot be empty"));
        }
        if val.chars().any(|c| c.is_control()) {
            return Err(SharedError::InvalidFormat("[Service] contains illegal format (control characters)"));
        }
        Ok(Self(val))
    }
}