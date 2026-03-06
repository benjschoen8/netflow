use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

use shared::domain::SharedError;
use crate::domain::money::Money;
use crate::domain::ticker::Ticker;
use super::investment_type::InvestmentType;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Investment {
    ticker: Ticker,
    investment_type: InvestmentType,
    quantity: Decimal,
    unit_price: Money,
}

impl Investment {
    pub fn new(
        ticker: Ticker,
        investment_type: InvestmentType,
        quantity: Decimal,
        unit_price: Money,
    ) -> Result<Self, SharedError> {
        if quantity.is_sign_negative() {
            return Err(SharedError::InvalidFormat(
                "[Investment] quantity cannot be negative"
            ));
        }
        Ok(Self { ticker, investment_type, quantity, unit_price })
    }

    pub fn ticker(&self) -> &Ticker { &self.ticker }
    pub fn investment_type(&self) -> &InvestmentType { &self.investment_type } // fixed: was security_type → SecurityType
    pub fn quantity(&self) -> Decimal { self.quantity }
    pub fn unit_price(&self) -> &Money { &self.unit_price }

    pub fn market_value(&self) -> Result<Money, SharedError> {
        let total = self.unit_price.amount
            .checked_mul(self.quantity)
            .ok_or(SharedError::Operational(
                "[Investment] arithmetic overflow calculating market value" // fixed
            ))?;
        Money::new(total, self.unit_price.currency)
    }

    pub fn update_price(&mut self, new_price: Money) {
        self.unit_price = new_price;
    }

    pub fn add_quantity(&mut self, amount: Decimal) -> Result<(), SharedError> {
        if amount.is_sign_negative() {
            return Err(SharedError::InvalidFormat(
                "[Investment] cannot add negative quantity" // fixed
            ));
        }
        self.quantity = self.quantity
            .checked_add(amount)
            .ok_or(SharedError::Operational(
                "[Investment] arithmetic overflow adding quantity" // fixed
            ))?;
        Ok(())
    }

    pub fn remove_quantity(&mut self, amount: Decimal) -> Result<(), SharedError> {
        if amount > self.quantity {
            return Err(SharedError::Operational(
                "[Investment] cannot remove more than held quantity" // fixed
            ));
        }
        self.quantity = self.quantity
            .checked_sub(amount)
            .ok_or(SharedError::Operational(
                "[Investment] arithmetic underflow removing quantity" // fixed
            ))?;
        Ok(())
    }
}
