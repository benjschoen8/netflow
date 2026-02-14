use chrono::{Duration, Utc};

use crate:shared::user_id::UserId;
use crate::role::Role;
use crate::time_frame::TimeFrame;
use crate::service::Service;
use crate::jwt_id::JwtId;

#[derive(Debug, Clone)]
pub struct AccessClaim {
    id: JwtId,
    // Standard JWT Claims (RFC 7519)
    subject: UserId,
    role: Role,
    time_frame: TimeFrame,
    service: Service,
}

impl AccessClaim {
    pub fn new(
        id: JwtId,
        subject: UserId, 
        role: Role, 
        time_frame: TimeFrame, 
        service: Service
    ) -> Self {
        Self {
            id, 
            subject, 
            role, 
            time_frame, 
            service 
        }
    }

    pub fn id(&self) -> JwtId {
        self.id
    }

    pub fn subject(&self) -> UserId {
        self.subject
    }

    pub fn role(&self) -> Role {
        self.role
    }

    pub fn service(&self) -> &Service {
        &self.service
    }

    pub fn time_frame(&self) -> &TimeFrame {
        &self.time_frame
    }

    pub fn is_active(&self) -> bool {
        self.time_frame.is_active()
    }
}
