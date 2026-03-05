use shared::domain::SharedError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    pub fn new(val: String) -> Result<Self, SharedError> {
        if val.is_empty(){
            return Err(SharedError::Empty("[Password:val] cannot be empty"));
        }
        Ok(Self(val))
    }
}