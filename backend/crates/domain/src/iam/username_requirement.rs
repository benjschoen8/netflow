use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct UsernameRequirement {
    pub min_username_len: usize,
    pub max_username_len: usize,
    pub allow_underscores: bool,
    pub allow_numeric: bool,
}