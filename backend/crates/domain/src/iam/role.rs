#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
