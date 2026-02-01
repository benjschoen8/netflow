use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountName(String);

Impl AccountName{
    pub fn new(name: String) -> Self {Self(name)}
}