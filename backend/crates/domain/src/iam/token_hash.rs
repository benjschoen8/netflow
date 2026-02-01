use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, DeriveValueType)]
pub struct TokenHash(String);

impl TokenHash {
    pub fn new(hash: impl Into<String>) -> Self {
        let h = hash.into();
        if h.trim().is_empty() {
            panic!("TokenHash cannot be empty");
        }
        Self(h)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl PartialEq<&str> for TokenHash {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}