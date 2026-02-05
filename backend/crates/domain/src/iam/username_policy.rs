use serde::Deserialize;

use crate::iam::policy::Policy;

#[derive(Debug, Deserialize, Clone)]
pub struct UsernamePolicy {
    pub min_len: usize,
    pub max_len: usize,
    pub illegal_characters: Vec<char>,
}

impl Policy for UsernamePolicy {
    fn subject(&self) -> &'static str { "username" }
    fn min_len(&self) -> usize { self.min_len }
    fn max_len(&self) -> usize { self.max_len }
    fn require_uppercase(&self) -> bool { false }
    fn require_lowercase(&self) -> bool { false }
    fn require_number(&self) -> bool { false }
    fn require_symbol(&self) -> bool { false }
    fn illegal_characters(&self) -> &[char] { &self.illegal_characters }
}