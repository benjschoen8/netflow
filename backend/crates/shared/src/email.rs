use crate::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email {
    address: String,
    sub_address: Option<String>,
    domain: String,
}

impl Email {
    pub fn parse(raw_email: String) -> Result<Self, SharedError> {
        if raw_email.trim().is_empty() {
            return Err(SharedError::Empty("[Email] cannot be empty"));
        }

        let (full_local, domain) = raw_email
            .split_once('@')
            .ok_or_else(|| SharedError::InvalidFormat("[Email] Missing '@' symbol"))?;

        if full_local.is_empty() {
            return Err(SharedError::InvalidFormat("[Email] Missing local part"));
        }
        if domain.is_empty() {
            return Err(SharedError::InvalidFormat("[Email] Missing domain part"));
        }

        let (address, sub_address) = match full_local.split_once('+') {
            Some((local, sub)) => (local.to_string(), Some(sub.to_string())),
            None => (full_local.to_string(), None),
        };

        Ok(Self {
            address,
            sub_address,
            domain: domain.to_string(),
        })
    }

    pub fn restore(local: String, sub: Option<String>, domain: String) -> Result<Self, SharedError> {
        Ok(Self {
            address: local,
            sub_address: sub,
            domain,
        })
    }

    pub fn full_address(&self) -> String {
        match &self.sub_address {
            Some(sub) => format!("{}+{}@{}", self.address, sub, self.domain),
            None => format!("{}@{}", self.address, self.domain),
        }
    }

    pub fn unique_address(&self) -> String {
        format!("{}@{}", self.address, self.domain)
    }
}