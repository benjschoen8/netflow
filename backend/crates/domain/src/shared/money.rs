use rust_decimal::Decimal;
use std::ops::{Add, Sub};

use crate::shared::shared_error::SharedError;
use crate::shared::currency::Currency;

#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn add(&self, other: &Self) -> Result<Self, SharedError> {
        self.ensure_same_currency(other)?;

        let new_amount = self.amount.checked_add(other.amount)
            .ok_or(SharedError::Operational("[Money:add] arithmetic overflow"))?;

        Self::new(new_amount, self.currency)
    }

    pub fn sub(&self, other: &Self) -> Result<Self, SharedError> {
        self.ensure_same_currency(other)?;

        let new_amount = self.amount.checked_sub(other.amount)
            .ok_or(SharedError::Operational("[Money:sub] arithmetic underflow"))?;

        Self::new(new_amount, self.currency)
    }

    fn ensure_same_currency(&self, other: &Self) -> Result<(), SharedError> {
        if self.currency != other.currency {
            return Err(SharedError::Operational("[Money] currency mismatch"));
        }
        Ok(())
    }
}
