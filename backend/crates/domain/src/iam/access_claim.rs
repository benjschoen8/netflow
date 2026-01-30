use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

// --- INTEGRATION: Import types from Shared Kernel ---
use crate::shared::user_id::UserId;
use crate::shared::role::Role;
use crate::shared::service::Service;
use crate::iam::time_frame::TimeFrame;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessClaim {
    // Standard JWT Claims (RFC 7519)
    pub subject: UserId,
    pub role: Role,
    pub time_frame: TimeFrame,
    pub service: Service,
}

impl AccessClaim {
    pub fn new(subject: UserId, role: Role, time_frame: TimeFrame, service: Service) -> Self {
        Self {
            subject,
            role,
            time_frame,
            service,
        }
    }
}
