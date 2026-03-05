use std::fmt;
use serde::{Serialize, Deserialize};

use shared::domain::SharedError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletName(String);

impl WalletName {
    pub fn new(val: String) -> Result<Self, SharedError> {
        let trimmed = val.trim().to_string();
        if trimmed.is_empty() {
            return Err(SharedError::Empty("[WalletName] cannot be empty"));
        }
        if trimmed.chars().any(|c| c.is_control()) {
            return Err(SharedError::InvalidFormat(
                "[WalletName] contains illegal characters"
            ));
        }
        Ok(Self(trimmed))
    }

    pub fn value(&self) -> &str { &self.0 }
}

impl fmt::Display for WalletName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
