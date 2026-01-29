use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq)]
pub enum IamError {
    // --- USERNAME RULES ---
    #[error("Username is already taken")]
    UsernameExists,

    #[error("Username cannot be empty")]
    UsernameEmpty,

    #[error("Username is too short")]
    UsernameTooShort,

    #[error("Username is too long")]
    UsernameTooLong,

    #[error("Username contains illegal characters (only a-z, 0-9, and _ allowed)")]
    UsernameIllegalChar,

    #[error("Username cannot start or end with an underscore")]
    UsernameInvalidFormat,

    // --- PASSWORD RULES ---
    #[error("Password cannot be empty")]
    PasswordEmpty,

    #[error("Password is too short")]
    PasswordTooShort,

    #[error("Password is too long (limit 128 chars)")]
    PasswordTooLong,

    #[error("Password requires at least one uppercase letter")]
    PasswordRequiresUppercase,

    #[error("Password requires at least one lowercase letter")]
    PasswordRequiresLowercase,

    #[error("Password requires at least one number")]
    PasswordRequiresNumber,

    #[error("Password requires at least one symbol")]
    PasswordRequiresSymbol,

    #[error("Password cannot contain whitespace")]
    PasswordContainsWhitespace,

    // --- SYSTEM / IDENTITY ---
    #[error("User not found")]
    UserNotFound,

    #[error("Invalid password provided during login")]
    InvalidCredentials,

    #[error("Cryptographic proof generation failed")]
    CryptoSystemError,
}