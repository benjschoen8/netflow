use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountName(String);

impl AccountName{
    pub fn new(name: String) -> Self {Self(name)}
}