use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventTimestamp {
    value: DateTime<Utc>,
}

impl EventTimestamp {
    pub fn now() -> Self {
        Self { value: Utc::now() }
    }

    pub fn from(dt: DateTime<Utc>) -> Self {
        Self { value: dt }
    }

    pub fn datetime(&self) -> DateTime<Utc> {
        self.value
    }
}