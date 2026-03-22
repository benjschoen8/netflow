//! `Statement` — a credit card billing cycle record.
//!
//! Created when `close_statement` is called. Tracks the full lifecycle of
//! a billing cycle: what was charged, what the closing balance was, how much
//! has been paid since, and whether it is fully settled.

use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::account_id::AccountId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statement {
    pub id:                Uuid,
    pub account_id:        AccountId,
    /// First day of the billing cycle (day after previous statement close,
    /// or account open date for the first statement).
    pub cycle_start:       NaiveDate,
    /// Last day of the billing cycle — the statement date.
    pub cycle_end:         NaiveDate,
    /// Total outstanding balance at the moment the statement closed.
    pub statement_balance: Decimal,
    /// Optional minimum payment required.
    pub minimum_payment:   Option<Decimal>,
    /// Sum of all charges recorded during this cycle.
    pub total_charged:     Decimal,
    /// Sum of all payments applied to this statement after it closed.
    pub total_paid:        Decimal,
    /// True when total_paid >= statement_balance.
    pub is_settled:        bool,
    /// When this statement was created.
    pub created_at:        chrono::DateTime<Utc>,
}

impl Statement {
    /// Create a new open statement at close time.
    pub fn new(
        account_id:        AccountId,
        cycle_start:       NaiveDate,
        cycle_end:         NaiveDate,
        statement_balance: Decimal,
        minimum_payment:   Option<Decimal>,
        total_charged:     Decimal,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            account_id,
            cycle_start,
            cycle_end,
            statement_balance,
            minimum_payment,
            total_charged,
            total_paid: Decimal::ZERO,
            is_settled: statement_balance.is_zero(),
            created_at: Utc::now(),
        }
    }

    /// Apply a payment to this statement. Recalculates `is_settled`.
    pub fn apply_payment(&mut self, amount: Decimal) {
        self.total_paid += amount;
        self.is_settled = self.total_paid >= self.statement_balance;
    }

    /// Remaining balance after payments.
    pub fn remaining(&self) -> Decimal {
        (self.statement_balance - self.total_paid).max(Decimal::ZERO)
    }
}
