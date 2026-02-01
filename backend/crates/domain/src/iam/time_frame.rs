use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeFrame {
    pub issued: usize,
    pub expiration: usize,
}

impl TimeFrame {
    pub fn new(issued: usize, expiration: usize) -> Self {
        Self { issued, expiration}
    }
}
