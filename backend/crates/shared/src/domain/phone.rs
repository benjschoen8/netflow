use crate::domain::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Phone(String);

impl Phone {
    pub fn new(value: String) -> Result<Self, SharedError> {
        if value.is_empty() {
            return Err(SharedError::InvalidFormat("[Phone] cannot be empty"));
        }

        Ok(Self(value.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Phone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}