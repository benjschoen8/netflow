use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

use shared::domain::SharedError;
use crate::domain::currency::Currency;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Liability {
    pub amount: Decimal,
    pub currency: Currency,
}

impl Liability {
    pub fn amount(&self) -> Decimal {
        return &self.decimal
    }

    pub fn currency(&self) -> Currency {
        return &self.currency
    }

    pub fn new(amount: Decimal, currency: Currency) -> Result<Self, SharedError> {
        if amount.is_sign_negative() {
            return Err(SharedError::InvalidFormat(
                "[Liability:amount] contains invalid state (negative value)"
            ));
        }
        Ok(Self { amount, currency })
    }

    pub fn add(&self, other: &Self) -> Result<Self, SharedError> {
        self.ensure_same_currency(other)?;
        let new_amount = self.amount
            .checked_add(other.amount)
            .ok_or(SharedError::Operational("[Liability:add] arithmetic overflow"))?;
        Self::new(new_amount, self.currency)
    }

    pub fn sub(&self, other: &Self) -> Result<Self, SharedError> {
        self.ensure_same_currency(other)?;
        let new_amount = self.amount
            .checked_sub(other.amount)
            .ok_or(SharedError::Operational("[Liability:sub] arithmetic underflow"))?;
        Self::new(new_amount, self.currency)
    }

    fn ensure_same_currency(&self, other: &Self) -> Result<(), SharedError> {
        if self.currency != other.currency {
            return Err(SharedError::Operational("[Liability] currency mismatch"));
        }
        Ok(())
    }
}
