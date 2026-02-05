use serde::Deserialize;

use crate::iam::policy::Policy;

#[derive(Debug, Deserialize, Clone)]
pub struct PasswordPolicy {
    pub min_len: usize,
    pub max_len: usize,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_number: bool,
    pub require_symbol: bool,
    pub illegal_characters: Vec<char>,
}

impl Policy for PasswordPolicy {
    fn subject(&self) -> &'static str { "password" }
    fn min_len(&self) -> usize { self.min_len }
    fn max_len(&self) -> usize { self.max_len }
    fn require_uppercase(&self) -> bool { self.require_uppercase }
    fn require_lowercase(&self) -> bool { self.require_lowercase }
    fn require_number(&self) -> bool { self.require_number }
    fn require_symbol(&self) -> bool { self.require_symbol }
    fn illegal_characters(&self) -> &[char] { &self.illegal_characters }
}