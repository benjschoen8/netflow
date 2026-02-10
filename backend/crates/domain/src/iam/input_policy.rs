use crate::iam::iam_error::IamError;

#[derive(Debug, Clone)]
pub struct InputPolicy {
    subject: &'static str,
    min_len: Option<usize>,
    max_len: Option<usize>,
    require_uppercase: bool,
    require_lowercase: bool,
    require_number: bool,
    require_symbol: bool,
    all_numbers: bool,
    illegal_characters: Vec<char>,
}


impl InputPolicy {
    pub fn validate(&self, input: &str) -> Result<(), IamError> {
        let min = self.min_len.unwrap_or(0);
        if input.is_empty() {
            if min > 0 {
                return Err(IamError::PolicyViolation(format!("[{}] is required", self.subject)));
            }
            return Ok(());
        }

        let mut count = 0;
        let mut has_upper = false;
        let mut has_lower = false;
        let mut has_digit = false;
        let mut has_symbol = false;

        for c in input.chars() {
            count += 1;

            if self.illegal_characters.contains(&c) || c.is_control() {
                return Err(IamError::PolicyViolation(format!(
                    "[InputPolicy] contains illegal character: '{}'", self.subject
                )));
            }

            if self.all_numbers && !c.is_ascii_digit() {
                 return Err(IamError::PolicyViolation(format!(
                    "[InputPolicy] must contain only digits: '{}'", self.subject
                )));
            }

            if c.is_uppercase() { has_upper = true; }
            else if c.is_lowercase() { has_lower = true; }
            else if c.is_numeric() { has_digit = true; }
            else if !c.is_alphanumeric() { has_symbol = true; }
        }

        self.evaluate_constraints(count, has_upper, has_lower, has_digit, has_symbol)
    }

    fn evaluate_constraints(&self, count: usize, upper: bool, lower: bool, digit: bool, symbol: bool) -> Result<(), IamError> {
        let sub = self.subject;

        if let Some(min) = self.min_len {
            if count < min { return Err(IamError::PolicyViolation(format!("[InputPolicy] too short: {}", sub))); }
        }
        if let Some(max) = self.max_len {
            if count > max { return Err(IamError::PolicyViolation(format!("[InputPolicy] too long: {}", sub))); }
        }

        if self.require_lowercase && !lower { return Err(IamError::PolicyViolation(format!("[InputPolicy] missing lowercase: {}", sub))); }
        if self.require_uppercase && !upper { return Err(IamError::PolicyViolation(format!("[InputPolicy] missing uppercase: {}", sub))); }
        if self.require_number && !digit { return Err(IamError::PolicyViolation(format!("[InputPolicy] missing number: {}", sub))); }
        if self.require_symbol && !symbol { return Err(IamError::PolicyViolation(format!("[InputPolicy] missing symbol: {}", sub))); }

        Ok(())
    }

    pub fn subject(&self) -> &'static str {
        return self.subject
    }
}