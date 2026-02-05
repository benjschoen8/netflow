pub trait Policy {
    fn subject(&self) -> &'static str;
    fn min_len(&self) -> usize;
    fn max_len(&self) -> usize;
    fn require_uppercase(&self) -> bool;
    fn require_lowercase(&self) -> bool;
    fn require_number(&self) -> bool;
    fn require_symbol(&self) -> bool;
    fn illegal_characters(&self) -> &[char];
}