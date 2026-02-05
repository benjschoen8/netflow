use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum IamError {
    #[error("{0} cannot be empty")]
    Empty(&'static str),

    #[error("{0} is too short (min: {1})")]
    TooShort(&'static str, usize),

    #[error("{0} is too long (max: {1})")]
    TooLong(&'static str, usize),

    #[error("{0} contains illegal characters: {1:?}")]
    IllegalCharacter(&'static str, Vec<char>),

    #[error("{0} contains a security-violating control character")]
    SecurityViolation(&'static str),

    #[error("{0} requires at least one uppercase letter")]
    MissingUppercase(&'static str),

    #[error("{0} requires at least one lowercase letter")]
    MissingLowercase(&'static str),

    #[error("{0} requires at least one number")]
    MissingNumber(&'static str),

    #[error("{0} requires at least one symbol")]
    MissingSymbol(&'static str),

    #[error("Username is already taken")]
    UsernameExists,

    #[error("User not found")]
    UserNotFound,

    #[error("Invalid credentials provided")]
    InvalidCredentials,
}