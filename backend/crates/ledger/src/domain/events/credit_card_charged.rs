use serde::Serialize;
use shared::domain::UserId;
use shared::domain::EventMetadata;
use shared::impl_domain_event;
use crate::domain::account_id::AccountId;
use crate::domain::liability::Liability;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CreditCardCharged {
    pub metadata: EventMetadata,
    pub user_id: UserId,
    pub account_id: AccountId,
    pub amount: Liability,
}

impl CreditCardCharged {
    pub fn new(user_id: UserId, account_id: AccountId, amount: Liability) -> Self {
        Self { metadata: EventMetadata::now(), user_id, account_id, amount }
    }
}

impl_domain_event!(CreditCardCharged, "credit_card_charged", "ledger", "ledger");
