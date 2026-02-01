use serde::{Deserialize, Serialize};

use crate::shared::user_id::UserId;
use crate::iam::time_frame::TimeFrame;
use crate::iam::token_hash::TokenHash;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefreshToken {
    token_hash: TokenHash,
    user_id: UserId,
    device_name: Option<String>,
    ip_address: Option<String>,
    time_frame: TimeFrame,
}

impl RefreshToken {
    pub fn new(
        token_hash: TokenHash, 
        user_id: UserId, 
        device_name: Option<String>,
        ip_address: Option<String>,
        time_frame: TimeFrame
    ) -> Self {
        Self {
            token_hash,
            user_id,
            device_name,
            ip_address,
            time_frame,
        }
    }

    pub fn token_hash(&self) -> &str {
        &self.token_hash
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn device_name(&self) -> Option<&str> {
        self.device_name.as_deref()
    }

    pub fn is_valid(&self) -> bool {
        self.time_frame.is_active()
    }
}