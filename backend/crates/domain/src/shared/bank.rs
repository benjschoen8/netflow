use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, DeriveValueType)]
pub struct Bank(String);

impl Bank {
    pub fn new(name: impl Into<String>) -> Self {
        let n = name.into();
        if n.trim().is_empty() {
            panic!("Bank name cannot be empty");
        }
        Self(n)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}