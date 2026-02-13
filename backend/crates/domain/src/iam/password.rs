use crate::shared::shared_error:SharedError;

#[derive(Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    pub fn new(val: String) -> Result<Self, SharedError> {
        if val.is_empty(){
            return Err(SharedError::Empty("[Password:val] cannot be empty"));
        }
        Ok(Self(val))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}