use crate::domain::sanitizable::Sanitizable;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum SharedError {
    #[error("{0}")]
    Empty(&'static str),

    #[error("{0}")]
    InvalidFormat(&'static str),
    
    #[error("{0}")]
    Operational(&'static str),

    #[error("{0}")]
    EventPublishFailed(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

impl Sanitizable for SharedError {
    fn safe_message(&self) -> String {
        match self {
            SharedError::Empty(_) => "A required field was missing".to_string(),
            SharedError::InvalidFormat(_) => "The provided data format is invalid".to_string(),
            SharedError::Operational(_) =>
                "The requested operation could not be completed due to a logical conflict".to_string(),
            SharedError::EventPublishFailed(_) => "The event could not be published".to_string(),
            SharedError::Database(_) => "A storage error occurred".to_string(),
            SharedError::Serialization(_) => "A data processing error occurred".to_string(),
        }
    }
}