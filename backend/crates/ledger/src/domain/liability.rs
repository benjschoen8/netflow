use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

use shared::domain::SharedError;
use crate::domain::money::Money;
use crate::domain::monthly_day::MonthlyDay;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Liability {
    principal: Money,

    outstanding: Money,

    creditor: String,

    interest_rate: Option<Decimal>,

    /// Recurring due date — dayof month e.g. MonthlyDay(15)
    due_date: Option<MonthlyDay>,

    maturity_date: Option<chrono::NaiveDate>,

    minimum_payment: Option<Money>,

    overdue: bool,
}

impl Liability {
    pub fn new(
        principal: Money,
        creditor: String,
        interest_rate: Option<Decimal>,
        due_date: Option<MonthlyDay>,
        maturity_date: Option<chrono::NaiveDate>,
        minimum_payment: Option<Money>,
    ) -> Result<Self, SharedError> {
        if creditor.trim().is_empty() {
            return Err(SharedError::Empty("[Liability] creditor cannot be empty"));
        }
        if let Some(rate) = interest_rate {
            if rate.is_sign_negative() {
                return Err(SharedError::InvalidFormat(
                    "[Liability] interest rate cannot be negative"
                ));
            }
        }
        let outstanding = principal.clone();
        Ok(Self {
            principal,
            outstanding,
            creditor,
            interest_rate,
            due_date,
            maturity_date,
            minimum_payment,
            overdue: false,
        })
    }

    pub fn principal(&self) -> &Money { &self.principal }
    pub fn outstanding(&self) -> &Money { &self.outstanding }
    pub fn creditor(&self) -> &str { &self.creditor }
    pub fn interest_rate(&self) -> Option<Decimal> { self.interest_rate }
    pub fn due_date(&self) -> Option<MonthlyDay> { self.due_date }
    pub fn maturity_date(&self) -> Option<chrono::NaiveDate> { self.maturity_date }
    pub fn minimum_payment(&self) -> Option<&Money> { self.minimum_payment.as_ref() }
    pub fn is_overdue(&self) -> bool { self.overdue }
    pub fn is_settled(&self) -> bool { self.outstanding.amount.is_zero() }

    pub fn mark_overdue(&mut self) { self.overdue = true; }
    pub fn mark_current(&mut self) { self.overdue = false; }

    pub fn make_payment(&self, payment: &Money) -> Result<Self, SharedError> {
        if payment.amount > self.outstanding.amount {
            return Err(SharedError::Operational(
                "[Liability] payment exceeds outstanding balance"
            ));
        }
        Ok(Self {
            outstanding: self.outstanding.sub(payment)?,
            ..self.clone()
        })
    }

    pub fn accrue_interest(&self) -> Result<Self, SharedError> {
        let Some(rate) = self.interest_rate else {
            return Ok(self.clone());
        };
        let monthly_rate = rate / Decimal::from(1200);
        let interest = self.outstanding.amount
            .checked_mul(monthly_rate)
            .ok_or(SharedError::Operational(
                "[Liability] overflow accruing interest"
            ))?;
        let new_amount = self.outstanding.amount
            .checked_add(interest)
            .ok_or(SharedError::Operational(
                "[Liability] overflow adding interest to outstanding"
            ))?;
        Ok(Self {
            outstanding: Money::new(new_amount, self.outstanding.currency)?,
            ..self.clone()
        })
    }
}
