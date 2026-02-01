use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountNumber(String);

impl AccountNumber{
    pub fn new(name: String) -> Self {Self(name)}
}
