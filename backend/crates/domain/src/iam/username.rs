use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, DeriveValueType)]
pub struct Username(String);

impl Username {
    pub fn new(raw: impl Into<String>) -> Result<Self, String> {
        let s = raw.into();
        let trimmed = s.trim();

        if trimmed.is_empty() {
            return Err("Username cannot be empty".to_string());
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Username> for String {
    fn from(val: Username) -> Self {
        val.0
    }
}