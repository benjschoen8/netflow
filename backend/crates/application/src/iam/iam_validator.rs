use crate::iam::iam_error::IamError;
use crate::iam::input_policy::InputPolicy;
use crate::shared::shared_error::SharedError;

#[derive(Debug, Clone, Copy, Default)]
pub struct IamValidator;

impl IamValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, policy: &InputPolicy, input: &str) -> Result<(), SharedError> {
        policy.validate(policy, input)?;
        Ok(())
    }
}