use crate::domain::iam_error::IamError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserStatus {
    Pending,
    Active,
    Suspended,
    Deactivated,
}

impl UserStatus {
    pub fn default() -> Self {
        Self::Pending
    }

    pub fn transition_to(&self, next: Self) -> Result<Self, IamError> {
        match (self, next) {
            (Self::Suspended, Self::Active) => Ok(next),
            (Self::Pending, Self::Active) => Ok(next),
            (Self::Active, Self::Suspended) | (Self::Active, Self::Deactivated) => Ok(next),
            _ => Err(IamError::InvalidStatusTransition),
        }
    }
}