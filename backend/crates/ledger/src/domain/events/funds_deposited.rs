use serde::Serialize;
use shared::domain::UserId;
use crate::domain::account_id::AccountId;
use crate::domain::money::Money;
use shared::domain::EventMetadata;
use shared::impl_domain_event;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FundsDeposited {
    pub metadata: EventMetadata,
    pub user_id: UserId,
    pub account_id: AccountId,
    pub amount: Money,
}

impl FundsDeposited {
    pub fn new(user_id: UserId, account_id: AccountId, amount: Money) -> Self {
        Self { metadata: EventMetadata::now(), user_id, account_id, amount }
    }
}

impl_domain_event!(FundsDeposited, "funds_deposited", "ledger", "ledger");
