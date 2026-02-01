use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Bank(String);

Impl Bank{
    pub fn new(name: String) -> Self {Self(name)}
}
