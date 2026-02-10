use serde::{Deserialize, Serialize};

use crate::shared::user_id::UserId;
use crate::shared::phone::Phone;
use crate::shared::email::Email;
use crate::iam::role::Role;
use crate::iam::username::Username;
use crate::iam::password_hash::PasswordHash;
use crate::iam::user_preferences::UserPreferences;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IamUser {
    id: UserId,
    username: Username,
    #[serde(skip_serializing)]
    password_hash: PasswordHash,
    email: Email,
    phone: Option<Phone>,
    role: Role,
    preferences: UserPreferences,
}

impl IamUser {
    pub fn new(
        id: UserId,
        username: Username, 
        password_hash: PasswordHash, 
        email: Email, 
        phone: Option<Phone>,
        role: Role,
        preferences: UserPreferences,
    ) -> Self {
        Self {
            id,
            username,
            password_hash,
            email,
            phone,
            role,
            preferences,
        }
    }

    pub fn id(&self) -> UserId {
        self.id
    }

    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn role(&self) -> Role {
        self.role
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn phone(&self) -> Option<&Phone> {
        self.phone.as_ref()
    }
    
    pub fn preferences(&self) -> &UserPreferences {
        &self.preferences
    }
}