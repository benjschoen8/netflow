use serde::Serialize;
use std::fmt;

use crate::domain::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize )]
pub struct Phone(String);

impl fmt::Display for Phone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Phone {
    pub fn new(value: String) -> Result<Self, SharedError> {
        if value.is_empty() {
            return Err(SharedError::InvalidFormat("[Phone] cannot be empty"));
        }

        Ok(Self(value.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}