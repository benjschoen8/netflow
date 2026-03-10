use serde::Serialize;
use shared::domain::UserId;
use crate::domain::account_id::AccountId;
use crate::domain::liability::Liability;
use shared::domain::EventMetadata;
use shared::impl_domain_event;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PaymentMade {
    pub metadata: EventMetadata,
    pub user_id: UserId,
    pub from_account_id: AccountId,
    pub debt_account_id: AccountId,
    pub amount: Liability,
}

impl PaymentMade {
    pub fn new(user_id: UserId, from_account_id: AccountId, debt_account_id: AccountId, amount: Liability) -> Self {
        Self { metadata: EventMetadata::now(), user_id, from_account_id, debt_account_id, amount }
    }
}

impl_domain_event!(PaymentMade, "payment_made", "ledger", "ledger");
