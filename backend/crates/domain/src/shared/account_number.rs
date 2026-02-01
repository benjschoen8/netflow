use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountNumber(String);

Impl AccountNumber{
    pub fn new(name: String) -> Self {Self(name)}
}
