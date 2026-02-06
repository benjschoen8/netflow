use crate::shared::currency::Currency;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};
use crate::shared::shared_error::SharedError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub amount: Decimal,
    pub currency: Currency,
}

impl Money {
    pub fn new(amount: Decimal, currency: Currency) -> Result<Self, SharedError> {
        if amount.is_sign_negative() {
            return Err(SharedError::InvalidFormat("[Money:amount] contains invalid state (negative value)"));
        }
        Ok(Self { amount, currency })
    }
}

impl Money {
    pub fn add(self, other: Self) -> Result<Self, SharedError> {
        if self.currency != other.currency {
            return Err(SharedError::Operational(
                "[Money:currency] contains operational failure (mismatch between currencies)"
            ));
        }

        let new_amount = self.amount.checked_add(other.amount)
            .ok_or(SharedError::Operational("[Money:amount] contains operational failure (arithmetic overflow)"))?;

        Ok(Self {
            amount: new_amount,
            currency: self.currency,
        })
    }

    pub fn sub(self, other: Self) -> Result<Self, SharedError> {
        if self.currency != other.currency {
            return Err(SharedError::Operational(
                "[Money:currency] contains operational failure (mismatch between currencies)"
            ));
        }

        let new_amount = self.amount.checked_sub(other.amount)
            .ok_or(SharedError::Operational("[Money:amount] contains operational failure (arithmetic underflow)"))?;

        if new_amount.is_sign_negative() {
            return Err(SharedError::Operational(
                "[Money:amount] contains operational failure (negative result)"
            ));
        }

        Ok(Self {
            amount: new_amount,
            currency: self.currency,
        })
    }
}
