use crate::shared::currency::Currency;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub amount: Decimal,
    pub currency: Currency,
}

impl Money {
    pub fn new(amount: Decimal, currency: Currency) -> Self {
        Self { amount, currency }
    }

    pub fn zero(currency: Currency) -> Self {
        Self {
            amount: Decimal::ZERO,
            currency,
        }
    }
}

// --- Math Logic ---

impl Add for Money {
    type Output = Money;

    fn add(self, other: Self) -> Self {
        if self.currency != other.currency {
            panic!(
                "Currency Mismatch: Cannot add {} to {}",
                self.currency, other.currency
            );
        }
        Money {
            amount: self.amount + other.amount,
            currency: self.currency,
        }
    }
}

impl Sub for Money {
    type Output = Money;

    fn sub(self, other: Self) -> Self {
        if self.currency != other.currency {
            panic!(
                "Currency Mismatch: Cannot subtract {} from {}",
                other.currency, self.currency
            );
        }
        Money {
            amount: self.amount - other.amount,
            currency: self.currency,
        }
    }
}
