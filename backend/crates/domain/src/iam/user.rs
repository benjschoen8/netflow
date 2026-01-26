use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub username: String, // No Email. Just Username.
    
    // Security: Still storing the Hash, never the password.
    pub password_hash: String,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    // Factory method updated: Removed email argument
    pub fn new(username: String, password_hash: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            password_hash,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}


#[test]
fn test_user_creation_success() {
    // 1. Arrange
    let username = "professor_taiwan".to_string();
    let hash = "argon2$v=19$m=4096,t=3,p=1$secret".to_string();

    // 2. Act
    let user = User::new(username.clone(), hash.clone());

    // 3. Assert - Check Fields
    assert_eq!(user.username, username);
    assert_eq!(user.password_hash, hash);
    
    // 4. Assert - Check Auto-Generated Fields
    assert!(!user.id.is_nil(), "User ID should be a valid UUID");
    
    // Check that created_at is recent (within last 5 seconds)
    let now = chrono::Utc::now();
    let diff = now.signed_duration_since(user.created_at);
    assert!(diff.num_seconds() < 5, "created_at should be set to 'now'");
}

#[test]
fn test_users_are_unique_instances() {
    // Even with same data, two new() calls should make different IDs
    let user1 = User::new("test".into(), "hash".into());
    let user2 = User::new("test".into(), "hash".into());

    assert_ne!(user1.id, user2.id);
}