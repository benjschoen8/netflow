use serde::Serialize;
use shared::domain::UserId;
use shared::domain::EventMetadata;
use crate::domain::account_id::AccountId;
use shared::impl_domain_event;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AccountMarkedCurrent {
    pub metadata: EventMetadata,
    pub user_id: UserId,
    pub account_id: AccountId,
}

impl AccountMarkedCurrent {
    pub fn new(user_id: UserId, account_id: AccountId) -> Self {
        Self { metadata: EventMetadata::now(), user_id, account_id }
    }
}

impl_domain_event!(AccountMarkedCurrent, "account_marked_current", "ledger", "ledger");
