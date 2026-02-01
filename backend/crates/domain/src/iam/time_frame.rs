use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeFrame {
    issued_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

impl TimeFrame {
    pub fn new(issued_at: DateTime<Utc>, duration: std::time::Duration) -> Self {
        let expires_at = issued_at + chrono::Duration::from_std(duration).unwrap();
        Self { issued_at, expires_at }
    }
    
    pub fn issued_at(&self) -> DateTime<Utc> {
        self.issued_at
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}
