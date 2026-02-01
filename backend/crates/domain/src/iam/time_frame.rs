use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeFrame {
    pub issued: usize,
    pub experation: usize,
}

impl TimeFrame {
    pub fn new(issued: usize, experation: usize) -> Self {
        Self { issued, experation}
    }
}
