use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::user_id::UserId;
use crate::shared::role::Role;
use crate::iam::username::Username;
use crate::iam::password_hash::PasswordHash;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IamUser {
    id: UserId,
    username: Username,
    #[serde(skip_serializing)]
    password_hash: PasswordHash,
    role: Role,
}

impl IamUser {
    pub fn new(username: Username, password_hash: PasswordHash) -> Self {
        Self {
            id: UserId::new(),
            username,
            password_hash,
            role: Role::User,
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
}