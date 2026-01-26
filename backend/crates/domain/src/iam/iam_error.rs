use thiserror::Error;

#[derive(Error, Debug)]
pub enum IamError {
    #[error("Username '{0}' is already taken")]
    UsernameExists(String),

    #[error("Username '{0}' is invalid (too short or illegal characters)")]
    InvalidUsername(String),

    #[error("User not found")]
    UserNotFound,
    
    #[error("Invalid password")]
    InvalidPassword,

    #[error("Internal System Error: {0}")]
    SystemError(String),
}