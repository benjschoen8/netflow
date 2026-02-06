use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::shared::shared_error::SharedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TimeMetadata(DateTime<Utc>);

impl TimeMetadata {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn from_raw(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }

    pub fn touch(self) -> Self {
        Self(Utc::now())
    }

    pub fn as_utc(&self) -> DateTime<Utc> {
        self.0
    }
}