use crate::shared::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email {
    address: String,
    sub_address: Option<String>,
    domain: String,
}

impl Email {
    pub fn new(
        address: String, 
        sub_address: Option<String>, 
        domain: String
    ) -> Result<Self, SharedError> {
        if address.is_empty() {
            return Err(SharedError::Empty("[Email:address] cannot be empty"));
        }

        if domain.is_empty() {
            return Err(SharedError::Empty("[Email:domain] cannot be empty"));
        }

        Ok(Self { 
            address, 
            sub_address: sub_address.filter(|s| !s.trim().is_empty()), 
            domain 
        })
    }

    pub fn full_address(&self) -> String {
        match &self.sub_address {
            Some(sub) => format!("{}+{}@{}", self.address, sub, self.domain),
            None => format!("{}@{}", self.address, self.domain),
        }
    }

    pub fn canonical_address(&self) -> String {
        format!("{}@{}", self.address, self.domain)
    }
}