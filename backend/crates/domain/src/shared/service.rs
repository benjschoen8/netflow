use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Service(String);

impl Service {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}