use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// --- INTEGRATION: Import types from Shared Kernel ---
use crate::shared::user_id::UserId;
use crate::shared::role::Role;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IamUser {
    // 1. Use the Domain Type
    pub id: UserId,

    pub username: String,

    // 2. SECURITY CRITICAL:
    // This tag ensures the hash is NEVER sent to the Frontend in JSON.
    #[serde(skip_serializing)]
    pub password_hash: String,

    // 3. Authorization Level
    pub role: Role,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IamUser {
    // Factory method
    pub fn new(username: String, password_hash: String) -> Self {
        Self {
            id: UserId::new_v4(),
            username,
            password_hash,
            // Default new users to the lowest permission level
            role: Role::User,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::role::Role;

    #[test]
    fn test_user_creation_integrates_shared_kernel() {
        let username = "testing".to_string();
        let hash = "secret_hash".to_string();

        let user = IamUser::new(username.clone(), hash.clone());

        assert_eq!(user.username, username);
        assert_eq!(user.password_hash, hash);
        assert_eq!(user.role, Role::User); // Verifying the default role
        assert!(!user.id.is_nil());
    }

    #[test]
    fn test_security_serialization() {
        let user = IamUser::new("hacker".into(), "secret_hash".into());
        let json = serde_json::to_string(&user).unwrap();

        // Assert: The JSON string should NOT contain "secret_hash"
        assert!(!json.contains("secret_hash"), "SECURITY FAIL: Password hash leaked in JSON!");
    }
}
