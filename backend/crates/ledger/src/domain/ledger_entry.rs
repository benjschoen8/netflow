//! `LedgerEntry` — a persistent record of every fund change on an account.
//!
//! This is a read-model / projection, not an aggregate. It is created by the
//! infrastructure layer whenever a transaction use case succeeds, and is
//! never mutated by domain logic (only the `label` and `description`
//! annotation fields are updatable, via a dedicated HTTP endpoint).

use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::domain::account_id::AccountId;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub id:          Uuid,
    pub account_id:  AccountId,
    pub entry_type:  EntryType,
    pub amount:      String,   // decimal string — avoids float precision issues in JSON
    pub currency:    String,
    pub occurred_at: DateTime<Utc>,
    /// Short user-provided label, e.g. "Lunch", "Salary", "Rent"
    pub label:       Option<String>,
    /// Longer free-text note, e.g. "McDonald's double cheeseburger + fries"
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryType {
    Deposit,
    Withdrawal,
    Charge,
    PaymentMade,
    PaymentReceived,
    InterestAccrued,
    StatementClosed,
}

impl EntryType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Deposit         => "deposit",
            Self::Withdrawal      => "withdrawal",
            Self::Charge          => "charge",
            Self::PaymentMade     => "payment_made",
            Self::PaymentReceived => "payment_received",
            Self::InterestAccrued => "interest_accrued",
            Self::StatementClosed => "statement_closed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "deposit"          => Some(Self::Deposit),
            "withdrawal"       => Some(Self::Withdrawal),
            "charge"           => Some(Self::Charge),
            "payment_made"     => Some(Self::PaymentMade),
            "payment_received" => Some(Self::PaymentReceived),
            "interest_accrued" => Some(Self::InterestAccrued),
            "statement_closed" => Some(Self::StatementClosed),
            _                  => None,
        }
    }

    /// Human-readable label for display.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Deposit         => "Deposit",
            Self::Withdrawal      => "Withdrawal",
            Self::Charge          => "Purchase",
            Self::PaymentMade     => "Payment",
            Self::PaymentReceived => "Payment Received",
            Self::InterestAccrued => "Interest",
            Self::StatementClosed => "Statement Closed",
        }
    }
}

impl LedgerEntry {
    pub fn new(
        account_id:  AccountId,
        entry_type:  EntryType,
        amount:      String,
        currency:    String,
        label:       Option<String>,
        description: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            account_id,
            entry_type,
            amount,
            currency,
            occurred_at: Utc::now(),
            label,
            description,
        }
    }
}
