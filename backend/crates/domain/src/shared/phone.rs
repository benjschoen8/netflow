use crate::shared::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Phone {
    region_code: String,
    number: String,
}

impl Phone {
    pub fn new(region_code: String, number: String) -> Result<Self, SharedError> {
        if region_code.is_empty() {
            return Err(SharedError::Empty("[Phone:region_code] cannot be empty"));
        }
        if !region_code.chars().all(|c| c.is_ascii_digit()) {
            return Err(SharedError::InvalidFormat("[Phone:region_code] must contain only digits"));
        }

        if number.is_empty() {
            return Err(SharedError::Empty("[Phone:number] cannot be empty"));
        }
        if !number.chars().all(|c| c.is_ascii_digit()) {
            return Err(SharedError::InvalidFormat("[Phone:number] must contain only digits"));
        }

        Ok(Self { region_code, number })
    }

    pub fn full_number(&self) -> String {
        format!("+{}{}", self.region_code, self.number)
    }
}