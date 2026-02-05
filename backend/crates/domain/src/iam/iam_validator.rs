use crate::iam::iam_error::IamError;
use crate::iam::policy::Policy;

#[derive(Debug, Clone, Copy, Default)]
pub struct IamValidator;

impl IamValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate<P: Policy>(&self, policy: &P, input: &str) -> Result<(), IamError> {
        self.check_presence(policy, input)?;
        self.check_illegal_chars(policy, input)?;
        self.check_length(policy, input)?;
        self.check_complexity(policy, input)?;

        Ok(())
    }

    fn check_presence<P: Policy>(&self, policy: &P, input: &str) -> Result<(), IamError> {
        if input.is_empty() {
            return Err(IamError::Empty(policy.subject()));
        }
        Ok(())
    }

    fn check_illegal_chars<P: Policy>(&self, policy: &P, input: &str) -> Result<(), IamError> {
        let subject = policy.subject();
        let forbidden_list = policy.illegal_characters();

        for c in input.chars() {
            if c == '\u{0000}' || c.is_control() {
                return Err(IamError::SecurityViolation(subject));
            }

            if forbidden_list.contains(&c) {
                return Err(IamError::IllegalCharacter(subject, forbidden_list.to_vec()));
            }
        }
        Ok(())
    }

    fn check_length<P: Policy>(&self, policy: &P, input: &str) -> Result<(), IamError> {
        let len = input.len();
        let subject = policy.subject();

        if len < policy.min_len() {
            return Err(IamError::TooShort(subject, policy.min_len()));
        }
        if len > policy.max_len() {
            return Err(IamError::TooLong(subject, policy.max_len()));
        }
        Ok(())
    }

    fn check_complexity<P: Policy>(&self, policy: &P, input: &str) -> Result<(), IamError> {
        let mut upper = false;
        let mut lower = false;
        let mut digit = false;
        let mut symbol = false;

        for c in input.chars() {
            if c.is_uppercase() { upper = true; }
            else if c.is_lowercase() { lower = true; }
            else if c.is_numeric() { digit = true; }
            else if !c.is_alphanumeric() { symbol = true; }
        }

        let subject = policy.subject();
        if policy.require_uppercase() && !upper { return Err(IamError::MissingUppercase(subject)); }
        if policy.require_lowercase() && !lower { return Err(IamError::MissingLowercase(subject)); }
        if policy.require_number() && !digit { return Err(IamError::MissingNumber(subject)); }
        if policy.require_symbol() && !symbol { return Err(IamError::MissingSymbol(subject)); }

        Ok(())
    }
}