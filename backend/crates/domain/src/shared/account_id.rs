use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountId(Uuid);

impl AccountId {
    pub fn new() -> Self { Self(Uuid::new_v4()) }
}
