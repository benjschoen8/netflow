use crate::domain::iam_error::IamError;
use crate::domain::input_policy::InputPolicy;
use shared::SharedError;

#[derive(Debug, Clone, Copy, Default)]
pub struct InputPolicyChecker;

impl InputPolicyChecker {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, policy: &InputPolicy, input: &str) {
        policy.validate(input);
    }
}