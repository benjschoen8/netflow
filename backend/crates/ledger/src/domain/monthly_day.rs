use chrono::{NaiveDate, Datelike};
use serde::{Serialize, Deserialize};

use shared::domain::SharedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonthlyDay(u8);

impl MonthlyDay {
    pub fn new(day: u8) -> Result<Self, SharedError> {
        if day < 1 || day > 31 {
            return Err(SharedError::InvalidFormat(
                "[MonthlyDay] must be between 1 and 31"
            ));
        }
        Ok(Self(day))
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn resolve(&self, year: i32, month: u32) -> NaiveDate {
        let days_in_month = days_in_month(year, month);
        let actual_day = (self.0 as u32).min(days_in_month);
        NaiveDate::from_ymd_opt(year, month, actual_day)
            .expect("date arithmetic produced invalid date")
    }

    pub fn resolve_current_month(&self) -> NaiveDate {
        let today = chrono::Utc::now().naive_utc().date();
        self.resolve(today.year(), today.month())
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    let first_of_next = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    };
    first_of_next
        .expect("valid date")
        .pred_opt()
        .expect("valid predecessor")
        .day()
}
CardType