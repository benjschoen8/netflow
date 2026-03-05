use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Role {
    Admin,
    User,
    Auditor,
}

impl Default for Role {
    fn default() -> Self {
        Role::User
    }
}
