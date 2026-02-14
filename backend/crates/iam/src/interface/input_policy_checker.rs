use crate::iam_error::IamError;
use crate::input_policy::InputPolicy;
use shared::SharedError;

#[derive(Debug, Clone, Copy, Default)]
pub struct InputPolicyChecker;

impl InputValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, policy: &InputPolicy, input: &str) -> Result<(), SharedError> {
        policy.validate(policy, input)?;
        Ok(())
    }
}