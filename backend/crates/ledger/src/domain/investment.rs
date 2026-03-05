use rust_decimal::Decimal;
use crate::domain::shared_error::SharedError;
use crate::domain::money::Money;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvestmentType {
    Stock,
    Etf,
    MutualFund,
    Bond,
    Crypto,
    Other(String),
}


#[derive(Debug, Clone, PartialEq)]
pub struct Investment {
    ticker: Ticker,
    security_type: SecurityType,

    quantity: Decimal,

    unit_price: Money,
}

impl Investment {
    pub fn new(
        ticker: Ticker,
        security_type: InvestmentType,
        quantity: Decimal,
        unit_price: Money,
    ) -> Result<Self, SharedError> {
        if quantity.is_sign_negative() {
            return Err(SharedError::InvalidFormat(
                "[Security] quantity cannot be negative"
            ));
        }
        Ok(Self {
            ticker,
            security_type,
            quantity,
            unit_price,
        })
    }

    pub fn ticker(&self) -> &Ticker {
        &self.ticker
    }

    pub fn security_type(&self) -> &SecurityType {
        &self.security_type
    }

    pub fn quantity(&self) -> Decimal {
        self.quantity
    }

    pub fn unit_price(&self) -> &Money {
        &self.unit_price
    }

    pub fn market_value(&self) -> Result<Money, SharedError> {
        let total = self.unit_price.amount
            .checked_mul(self.quantity)
            .ok_or(SharedError::Operational(
                "[Security] arithmetic overflow calculating market value"
            ))?;
        Money::new(total, self.unit_price.currency.clone())
    }

    pub fn update_price(&mut self, new_price: Money) {
        self.unit_price = new_price;
    }

    pub fn add_quantity(&mut self, amount: Decimal) -> Result<(), SharedError> {
        if amount.is_sign_negative() {
            return Err(SharedError::InvalidFormat(
                "[Security] cannot add negative quantity"
            ));
        }
        self.quantity = self.quantity
            .checked_add(amount)
            .ok_or(SharedError::Operational(
                "[Security] arithmetic overflow adding quantity"
            ))?;
        Ok(())
    }

    pub fn remove_quantity(&mut self, amount: Decimal) -> Result<(), SharedError> {
        if amount > self.quantity {
            return Err(SharedError::Operational(
                "[Security] cannot remove more than held quantity"
            ));
        }
        self.quantity = self.quantity
            .checked_sub(amount)
            .ok_or(SharedError::Operational(
                "[Security] arithmetic underflow removing quantity"
            ))?;
        Ok(())
    }
}
