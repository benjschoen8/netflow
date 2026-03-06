use std::fmt;
use serde::{ Serialize, Deserialize };

use shared::domain::SharedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpirationDate {
    month: u8,
    year: u16,
}

impl ExpirationDate {
    pub fn new(month: u8, year: u16) -> Result<Self, SharedError> {
        if month < 1 || month > 12 {
            return Err(SharedError::InvalidFormat(
                "[ExpirationDate] month must be between 1 and 12"
            ));
        }
        Ok(Self { month, year })
    }

    pub fn month(&self) -> u8 { self.month }
    pub fn year(&self) -> u16 { self.year }

    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now();
        let current_year = now.format("%Y").to_string().parse::<u16>().unwrap_or(0);
        let current_month = now.format("%m").to_string().parse::<u8>().unwrap_or(0);
        self.year < current_year
            || (self.year == current_year && self.month < current_month)
    }
}

impl fmt::Display for ExpirationDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}/{}", self.month, self.year % 100)
    }
}