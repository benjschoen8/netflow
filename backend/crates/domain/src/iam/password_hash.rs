use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use std::fmt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, DeriveValueType)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(hash: impl Into<String>) -> Self {
        let h = hash.into();
        if h.trim().is_empty() {
            panic!("PasswordHash cannot be empty");
        }
        Self(h)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for PasswordHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED]")
    }
}