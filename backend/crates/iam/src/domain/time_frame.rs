use chrono::{DateTime, Utc, Duration};
use crate::shared::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeFrame {
    issued_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

impl TimeFrame {
    pub fn new(issued_at: DateTime<Utc>, expires_at: DateTime<Utc>) -> Result<Self, SharedError> {
        if issued_at >= expires_at {
            return Err(SharedError::Operational(
                "[TimeFrame] expiration must be strictly after issuance".into()
            ));
        }
        Ok(Self { issued_at, expires_at })
    }

    pub fn create(duration: Duration) -> Result<Self, SharedError> {
        let now = Utc::now();
        
        if duration <= Duration::zero() {
             return Err(SharedError::Operational(
                 "[TimeFrame] duration must be positive".into()
             ));
        }

        let expires_at = now.checked_add_signed(duration)
            .ok_or_else(|| SharedError::Operational(
                "[TimeFrame] arithmetic overflow calculating expiration".into()
            ))?;

        Self::new(now, expires_at)
    }

    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        now >= self.issued_at && now < self.expires_at
    }
}