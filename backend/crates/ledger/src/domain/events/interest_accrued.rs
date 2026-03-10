use serde::Serialize;
use shared::domain::UserId;
use crate::domain::account_id::AccountId;
use crate::domain::liability::Liability;
use shared::domain::EventMetadata;
use shared::impl_domain_event;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct InterestAccrued {
    pub metadata: EventMetadata,
    pub user_id: UserId,
    pub account_id: AccountId,
    pub amount: Liability,
}

impl InterestAccrued {
    pub fn new(user_id: UserId, account_id: AccountId, amount: Liability) -> Self {
        Self { metadata: EventMetadata::now(), user_id, account_id, amount }
    }
}

impl_domain_event!(InterestAccrued, "interest_accrued", "ledger", "ledger");
