use serde::Serialize;
use shared::domain::UserId;
use crate::domain::account_id::AccountId;
use shared::domain::EventMetadata;
use shared::impl_domain_event;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MinimumPaymentMet {
    pub metadata: EventMetadata,
    pub user_id: UserId,
    pub account_id: AccountId,
}

impl MinimumPaymentMet {
    pub fn new(user_id: UserId, account_id: AccountId) -> Self {
        Self { metadata: EventMetadata::now(), user_id, account_id }
    }
}

impl_domain_event!(MinimumPaymentMet, "minimum_payment_met", "ledger", "ledger");
