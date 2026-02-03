use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PasswordRequirement {
    pub min_password_len: usize,
    pub max_password_len: usize,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_number: bool,
    pub require_symbol: bool,
}