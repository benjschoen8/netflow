use thiserror::Error;

#[derive(Error, Debug)]
pub enum IamError {
    // Business Rule: Uniqueness
    #[error("Username '{0}' is already taken")]
    UsernameExists(String),

    // Business Rule: Validation
    #[error("Username '{0}' is invalid (must be 3-20 alphanum chars)")]
    InvalidUsername(String),

    // Business Rule: Identity
    #[error("User not found")]
    UserNotFound,
    
    // Security
    #[error("Invalid password")]
    InvalidPassword,

    // Infrastructure / Unexpected
    // This allows the Domain to pass up DB errors without exposing SQL details
    #[error("Internal IAM Error: {0}")]
    SystemError(String),
}
