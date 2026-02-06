use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeFrame {
    issued_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

impl TimeFrame {
    pub fn new(issued_at: DateTime<Utc>, expires_at: DateTime<Utc>) -> Result<Self, SharedError> {
        if issued_at >= expires_at {
            return Err(SharedError::Operational(
                "[TimeFrame] contains operational failure (expiration must be after issuance)"
            ));
        }
        Ok(Self { issued_at, expires_at })
    }

    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        now >= self.issued_at && now < self.expires_at
    }
}
