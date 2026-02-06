#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Email {
    address: String,
    sub_address: Option<String>,
    domain: String,
}

impl Email {
    pub(crate) fn new(
        address: String, 
        sub_address: Option<String>, 
        domain: String
    ) -> Self {
        Self { 
            address, 
            sub_address, 
            domain 
        }
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