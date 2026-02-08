#[derive(Debug, Clone)]
pub struct InputPolicy {
    pub subject: &'static str,
    pub min_len: Option<usize>,
    pub max_len: Option<usize>,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_number: bool,
    pub require_symbol: bool,
    pub illegal_characters: Vec<char>,
}

impl Policy for DomainPolicy {
    fn subject(&self) -> &'static str { self.subject }
    fn min_len(&self) -> Option<usize> { self.min_len }
    fn max_len(&self) -> Option<usize> { self.max_len }
    fn require_uppercase(&self) -> bool { self.require_uppercase }
    fn require_lowercase(&self) -> bool { self.require_lowercase }
    fn require_number(&self) -> bool { self.require_number }
    fn require_symbol(&self) -> bool { self.require_symbol }
    fn illegal_characters(&self) -> &[char] { &self.illegal_characters }
}