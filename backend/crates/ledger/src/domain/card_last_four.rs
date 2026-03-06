use std::fmt;
use serde::{Serialize, Deserialize};

use shared::domain::SharedError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardLastFour(String);

impl CardLastFour {
    pub fn new(val: String) -> Result<Self, SharedError> {
        if val.len() != 4 {
            return Err(SharedError::InvalidFormat(
                "[CardLastFour] must be exactly 4 digits"
            ));
        }
        if !val.chars().all(|c| c.is_ascii_digit()) {
            return Err(SharedError::InvalidFormat(
                "[CardLastFour] must contain only digits"
            ));
        }
        Ok(Self(val))
    }

    pub fn value(&self) -> &str { &self.0 }
}

impl fmt::Display for CardLastFour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "••••{}", self.0)
    }
}