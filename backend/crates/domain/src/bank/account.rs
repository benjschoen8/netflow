use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// --- INTEGRATION: Import types from Shared Kernel ---
use crate::shared::user_id::UserId;
use crate::shared::account_id::AccountId;
use crate::shared::money::Money;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bank {
    // 1. Use the Domain Type
    pub user_id: UserId,
    pub account_id: AccountId,
    pub account_name: String,
    pub account_number: String,
    pub balance: Money,
}

impl Bank {
    // Factory method
    pub(crate) fn new(user_id: UserId, username: String, account_id: AccountId, account_name: String, account_number: String, balance: Money) -> Self {
        Self {
            user_id,
            username,
            account_id,
            account_name,
            account_number,
            balance,
        }
    }


}