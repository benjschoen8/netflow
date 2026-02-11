use crate::shared::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme(String);
impl Theme {

    pub fn new(val: String) -> Result<Self, SharedError> {
        if val.is_empty() {
            return Err(SharedError::Empty("[Theme:val]Theme val cannot be empty"));
        }

        if !val.chars().any(|c| c.is_control()) {
            return Err(SharedError::InvalidFormat("[Theme:val]Theme val invalid format"));
        }

        Ok(Self(val.to_string()))
    }
}