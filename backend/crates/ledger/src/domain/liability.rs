use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

use shared::domain::SharedError;
use crate::domain::currency::Currency;
use crate::domain::money::Money;
use crate::domain::balance::Balance;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Liability {
    amount: Decimal,
    currency: Currency,
}

impl Liability {
    pub fn new(amount: Decimal, currency: Currency) -> Result<Self, SharedError> {
        if amount.is_sign_negative() {
            return Err(SharedError::InvalidFormat(
                "[Liability] amount cannot be negative"
            ));
        }
        Ok(Self { amount, currency })
    }

    pub fn zero(currency: Currency) -> Self {
        Self { amount: Decimal::ZERO, currency }
    }

    pub fn amount(&self) -> Decimal { self.amount }
    pub fn currency(&self) -> Currency { self.currency }
    pub fn is_zero(&self) -> bool { self.amount.is_zero() }

    pub fn add(&self, other: &Self) -> Result<Self, SharedError> {
        if self.currency != other.currency {
            return Err(SharedError::Operational("[Liability:add] currency mismatch"));
        }
        let new_amount = self.amount
            .checked_add(other.amount)
            .ok_or(SharedError::Operational("[Liability:add] arithmetic overflow"))?;
        Self::new(new_amount, self.currency)
    }

    pub fn sub(&self, other: &Self) -> Result<Self, SharedError> {
        if self.currency != other.currency {
            return Err(SharedError::Operational("[Liability:sub] currency mismatch"));
        }
        if other.amount > self.amount {
            return Err(SharedError::Operational(
                "[Liability:sub] payment exceeds outstanding — use cross_sub if Money is valid"
            ));
        }
        let new_amount = self.amount
            .checked_sub(other.amount)
            .ok_or(SharedError::Operational("[Liability:sub] arithmetic underflow"))?;
        Self::new(new_amount, self.currency)
    }

    pub fn cross_sub(&self, other: &Money) -> Result<Balance, SharedError> {
        if self.currency != other.currency() {
            return Err(SharedError::Operational("[Liability:cross_sub] currency mismatch"));
        }
        if self.amount >= other.amount() {
            Ok(Balance::Debt(Liability::new(self.amount - other.amount(), self.currency)?))
        } else {
            Ok(Balance::Asset(Money::new(other.amount() - self.amount, self.currency)?))
        }
    }
}
