use crate::shared::shared_error::SharedError;
use phonenumber::PhoneNumber;
use std::str::FromStr;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct Phone {
    internal_value: PhoneNumber,
}

impl Eq for Phone {}

impl PartialOrd for Phone {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Phone {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_e164().cmp(&other.as_e164())
    }
}

impl FromStr for Phone {
    type Err = SharedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = phonenumber::parse(None, s)
            .map_err(|_| SharedError::InvalidFormat("[Phone] Invalid phone number format"))?;

        if !parsed.is_valid() {
            return Err(SharedError::InvalidFormat("[Phone] Number is impossible or invalid for its region"));
        }

        Ok(Self { internal_value: parsed })
    }
}

impl Phone {
    pub fn parse(s: &str) -> Result<Self, SharedError> {
        Self::from_str(s)
    }

    pub fn as_e164(&self) -> String {
        self.internal_value.format().mode(phonenumber::Mode::E164).to_string()
    }
}