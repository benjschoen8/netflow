use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

// --- INTEGRATION: Import types from Shared Kernel ---
use crate::shared::user_id::UserId;
use crate::shared::role::Role;
use crate::iam::time_frame::TimeFrame;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessClaim {
    // Standard JWT Claims (RFC 7519)
    subject: UserId,
    role: Role,
    time_frame: TimeFrame,
    service: String,
}

impl AccessClaim {
    pub fn new(subject: UserId, role: Role, time_frame: TimeFrame, service: String) -> Self {
        Self {
            subject,
            role,
            time_frame,
            service,
        }
    }

    pub fn subject(&self) -> UserId {
        self.subject
    }

    pub fn role(&self) -> Role {
        self.role
    }

    pub fn service(&self) -> &String {
        &self.service
    }

    pub fn time_frame(&self) -> &TimeFrame {
        &self.time_frame
    }

    pub fn is_active(&self) -> bool {
        self.time_frame.is_active()
    }
}
