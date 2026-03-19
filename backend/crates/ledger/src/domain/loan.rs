use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

use shared::domain::SharedError;
use crate::domain::liability::Liability;
use crate::domain::monthly_day::MonthlyDay;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Loan {
    principal: Liability,

    outstanding: Liability,

    creditor: String,

    interest_rate: Option<Decimal>,

    due_date: Option<MonthlyDay>,

    maturity_date: Option<chrono::NaiveDate>,

    minimum_payment: Option<Liability>,

    overdue: bool,
}

impl Loan {
    pub fn new(
        principal: Liability,
        creditor: String,
        interest_rate: Option<Decimal>,
        due_date: Option<MonthlyDay>,
        maturity_date: Option<chrono::NaiveDate>,
        minimum_payment: Option<Liability>,
    ) -> Result<Self, SharedError> {
        if creditor.trim().is_empty() {
            return Err(SharedError::Empty("[Loan] creditor cannot be empty"));
        }
        if let Some(rate) = interest_rate {
            if rate.is_sign_negative() {
                return Err(SharedError::InvalidFormat(
                    "[Loan] interest rate cannot be negative"
                ));
            }
        }
        if let Some(ref min) = minimum_payment {
            if min.currency() != principal.currency() {
                return Err(SharedError::Operational(
                    "[Loan] minimum payment currency must match principal"
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

    pub fn principal(&self) -> &Liability { &self.principal }
    pub fn outstanding(&self) -> &Liability { &self.outstanding }
    pub fn creditor(&self) -> &str { &self.creditor }
    pub fn interest_rate(&self) -> Option<Decimal> { self.interest_rate }
    pub fn due_date(&self) -> Option<MonthlyDay> { self.due_date }
    pub fn maturity_date(&self) -> Option<chrono::NaiveDate> { self.maturity_date }
    pub fn minimum_payment(&self) -> Option<&Liability> { self.minimum_payment.as_ref() }
    pub fn is_overdue(&self) -> bool { self.overdue }
    pub fn is_settled(&self) -> bool { self.outstanding.is_zero() }

    pub fn mark_overdue(&mut self) { self.overdue = true; }
    pub fn mark_current(&mut self) { self.overdue = false; }

    /// Returns a new Loan with outstanding reduced by payment.
    /// Err if payment exceeds outstanding.
    pub fn apply_payment(&self, payment: &Liability) -> Result<Self, SharedError> {
        Ok(Self {
            outstanding: self.outstanding.sub(payment)?,
            ..self.clone()
        })
    }

    /// Returns a new Loan with one month of interest applied.
    /// No-op if no interest rate is set.
    pub fn apply_interest(&self) -> Result<Self, SharedError> {
        let Some(rate) = self.interest_rate else {
            return Ok(self.clone());
        };
        let monthly_rate = rate / Decimal::from(1200);
        let interest = self.outstanding.amount()
            .checked_mul(monthly_rate)
            .ok_or(SharedError::Operational("[Loan] overflow accruing interest"))?;
        let new_amount = self.outstanding.amount()
            .checked_add(interest)
            .ok_or(SharedError::Operational("[Loan] overflow adding interest"))?;
        Ok(Self {
            outstanding: Liability::new(new_amount, self.outstanding.currency())?,
            ..self.clone()
        })
    }

    /// Restore a `Loan` from persistence, preserving both original principal
    /// and current outstanding balance (which may have been paid down).
    #[allow(clippy::too_many_arguments)]
    pub fn restore(
        principal: Liability,
        outstanding: Liability,
        creditor: String,
        interest_rate: Option<Decimal>,
        due_date: Option<MonthlyDay>,
        maturity_date: Option<chrono::NaiveDate>,
        minimum_payment: Option<Liability>,
        overdue: bool,
    ) -> Result<Self, SharedError> {
        if creditor.trim().is_empty() {
            return Err(SharedError::Empty("[Loan] creditor cannot be empty"));
        }
        Ok(Self {
            principal,
            outstanding,
            creditor,
            interest_rate,
            due_date,
            maturity_date,
            minimum_payment,
            overdue,
        })
    }
}
