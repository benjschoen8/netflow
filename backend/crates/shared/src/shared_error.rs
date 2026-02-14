#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum SharedError {
    #[error("{0}")]
    Empty(&'static str),

    #[error("{0}")]
    InvalidFormat(&'static str),
    
    #[error("{0}")]
    Operational(&'static str),
}

impl SharedError {
    pub fn sanitize(&self) -> String {
        match self {
            SharedError::Empty(_) => "A required field was missing".to_string(),
            SharedError::InvalidFormat(_) => "The provided data format is invalid".to_string(),
            SharedError::Operational(_) => 
                "The requested operation could not be completed due to a logical conflict".to_string(),
        }
    }
}