use uuid::Uuid;
use crate::domain::shared_error::SharedError;
use crate::domain::aggregate_root_id::AggregateRootId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserId(Uuid);

impl AggregateRootId for UserId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl UserId {
    pub fn restore(id: Uuid) -> Result<Self, SharedError> {
        if id.is_nil() {
            return Err(SharedError::InvalidFormat("[UserId] contains illegal format (all zeros)"));
        }
        Ok(Self(id))
    }
    
    pub fn create() -> Self { Self(Uuid::new_v4()) }
}