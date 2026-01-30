use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// --- INTEGRATION: Import types from Shared Kernel ---
use crate::shared::user_id::UserId;
use crate::shared::account_id::AccountId;
use crate::shared::account_name::AccountName;
use crate::shared::bank::Bank;
use crate::shared::account_number::AccountNumber;
use crate::shared::money::Money;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BankAccount {
    // 1. Use the Domain Type
    pub user_id: UserId,
    pub account_id: AccountId,
    pub account_name: AccountName,
    pub bank: Bank,
    pub account_number: AccountNumber,
    pub balance: Money,
}

impl BankAccount {
    // Factory method
    pub(crate) fn new(user_id: UserId, account_id: AccountId, account_name: AccountName, bank: Bank, account_number: AccountNumber, balance: Money) -> Self {
        Self {
            user_id,
            account_id,
            account_name,
            bank,
            account_number,
            balance,
        }
    }
}