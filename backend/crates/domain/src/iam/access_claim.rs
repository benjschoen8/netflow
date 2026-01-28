use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

// --- INTEGRATION: Import types from Shared Kernel ---
use crate::shared::user_id::UserId;
use crate::shared::role::Role;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessClaim {
    // Standard JWT Claims (RFC 7519)
    pub sub: UserId,        // Subject (Who?)
    pub exp: usize,         // Expiration (Unix Timestamp)
    pub iat: usize,         // Issued At (Unix Timestamp)
    pub iss: String,        // Issuer ("MyThesisSystem")
    
    // Custom Claims
    pub role: Role,
}

impl AccessClaim {
    pub fn new(user_id: UserId, role: Role, duration_seconds: i64) -> Self {
        let now = Utc::now();
        Self {
            sub: user_id,
            exp: (now + Duration::seconds(duration_seconds)).timestamp() as usize,
            iat: now.timestamp() as usize,
            iss: "MyThesisSystem".to_string(),
            role,
        }
    }
}
