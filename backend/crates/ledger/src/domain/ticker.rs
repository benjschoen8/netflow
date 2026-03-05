use shared::doamin::SharedError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticker(String);

impl Ticker {
    pub fn new(val: String) -> Result<Self, SharedError> {
        let upper = val.trim().to_uppercase();
        if upper.is_empty() {
            return Err(SharedError::Empty("[Ticker] cannot be empty"));
        }
        if upper.chars().any(|c| !c.is_alphanumeric() && c != '-' && c != '.') {
            return Err(SharedError::InvalidFormat(
                "[Ticker] contains illegal characters"
            ));
        }
        Ok(Self(upper))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
