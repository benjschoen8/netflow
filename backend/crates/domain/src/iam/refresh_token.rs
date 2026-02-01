use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// --- INTEGRATION: Import types from Shared Kernel ---
use crate::shared::user_id::UserId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefreshToken {
    // Security: We hash the token so if the DB is leaked, 
    // attackers cannot hijack the session.
    pub token_hash: String,
    
    pub user_id: UserId,
    
    pub device_name: Option<String>,
    pub ip_address: Option<String>,
    
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>, // Usually 30 days
    
    // Kill Switch: Explicitly revoked sessions (e.g., "Log out all devices")
    pub is_revoked: bool,
}

impl RefreshToken {
    // Helper to check validity
    pub fn is_valid(&self) -> bool {
        !self.is_revoked && self.expires_at > Utc::now()
    }
}
