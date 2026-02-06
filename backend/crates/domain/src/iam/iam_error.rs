use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum IamError {
    #[error("Username is already taken")]
    UsernameExists,

    #[error("User not found")]
    UserNotFound,

    #[error("Invalid credentials provided")]
    InvalidCredentials,
}