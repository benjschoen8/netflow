use crate::iam::iam_error::IamError;

#[derive(Debug, Clone, Copy)] // Copy is cheap for simple bools/usize
pub struct IamInputValidator {
    pub min_username_len: usize,
    pub max_username_len: usize,
    pub min_password_len: usize,
    pub max_password_len: usize,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_number: bool,
    pub require_symbol: bool,
}

impl Default for IamInputValidator {
    /// The Default "Strict" Configuration
    /// This is deterministic. It does not look at .env.
    /// The Application Layer is responsible for reading .env and overriding these if needed.
    fn default() -> Self {
        Self {
            min_username_len: 3,
            max_username_len: 30,
            min_password_len: 5,    // Default (Dev)
            max_password_len: 128,
            require_uppercase: true,
            require_lowercase: true,
            require_number: true,
            require_symbol: true,
        }
    }
}

impl IamInputValidator {
    /// Constructor that simply returns the default strict rules.
    pub fn new() -> Self {
        Self::default()
    }

    // --- USERNAME CHECKS ---

    pub fn validate_username(&self, username: &str) -> Result<(), IamError> {
        if username.trim().is_empty() {
            return Err(IamError::UsernameEmpty);
        }

        let len = username.len();
        if len < self.min_username_len {
            return Err(IamError::UsernameTooShort);
        }
        if len > self.max_username_len {
            return Err(IamError::UsernameTooLong);
        }

        for c in username.chars() {
            if !c.is_alphanumeric() && c != '_' {
                return Err(IamError::UsernameIllegalChar);
            }
        }

        if username.starts_with('_') || username.ends_with('_') {
            return Err(IamError::UsernameInvalidFormat);
        }

        Ok(())
    }

    // --- PASSWORD CHECKS ---

    pub fn validate_password_complexity(&self, password: &str) -> Result<(), IamError> {
        if password.is_empty() {
            return Err(IamError::PasswordEmpty);
        }

        if password.contains(char::is_whitespace) {
            return Err(IamError::PasswordContainsWhitespace);
        }

        let len = password.len();
        if len < self.min_password_len {
            return Err(IamError::PasswordTooShort);
        }
        if len > self.max_password_len {
            return Err(IamError::PasswordTooLong);
        }

        let mut has_upper = false;
        let mut has_lower = false;
        let mut has_digit = false;
        let mut has_symbol = false;

        for c in password.chars() {
            if c.is_uppercase() { has_upper = true; }
            else if c.is_lowercase() { has_lower = true; }
            else if c.is_numeric() { has_digit = true; }
            else { has_symbol = true; }
        }

        if self.require_uppercase && !has_upper {
            return Err(IamError::PasswordRequiresUppercase);
        }
        if self.require_lowercase && !has_lower {
            return Err(IamError::PasswordRequiresLowercase);
        }
        if self.require_number && !has_digit {
            return Err(IamError::PasswordRequiresNumber);
        }
        if self.require_symbol && !has_symbol {
            return Err(IamError::PasswordRequiresSymbol);
        }

        Ok(())
    }
}