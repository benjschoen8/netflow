use thiserror::Error;
use shared::{SharedError, Sanitizable};

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum IamError {
    #[error(transparent)]
    Shared(#[from] SharedError),

    #[error("Username is already taken")]
    UsernameExists,
    
    #[error("Email is already taken")]
    EmailExists,

    #[error("Password mismatched")]
    PasswordMismatch,

    #[error("User not found")]
    UserNotFound,

    #[error("Invalid credentials provided")]
    InvalidCredentials,
    
    #[error("Invalid status transition")]
    InvalidStatusTransition,

    #[error("Policy Violation: {0}")]
    PolicyViolation(String),
}

impl Sanitizable for IamError {
    fn safe_message(&self) -> String {
        match self {
            IamError::Shared(inner) => inner.safe_message(),

            IamError::UsernameExists => "This username is already registered.".to_string(),
            IamError::EmailExists => "This email is already registered.".to_string(),
            IamError::PasswordMismatch => "This password does not match the confirmation.".to_string(),
            IamError::PolicyViolation(msg) => format!("Security policy violation: {}", msg),
            
            IamError::UserNotFound | IamError::InvalidCredentials => 
                "Invalid username or password.".to_string(),

            IamError::InvalidStatusTransition => "Account validation failed. Please contact support.".to_string(),
        }
    }
}