use chrono::{DateTime, Utc, TimeZone};
use serde::{Deserialize, Serialize};
use crate::shared::shared_error::SharedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DomainDate(DateTime<Utc>);

impl DomainDate {
    pub fn new(y: i32, m: u32, d: u32) -> Result<Self, SharedError> {
        Utc.with_ymd_and_hms(y, m, d, 0, 0, 0)
            .single()
            .map(Self)
            .ok_or(SharedError::InvalidFormat(
                "[DomainDate:raw] contains illegal date values (non-existent day)"
            ))
    }

    pub fn as_utc(&self) -> DateTime<Utc> {
        self.0
    }
}