use serde::Serialize;
use shared::domain::UserId;
use shared::domain::EventMetadata;
use crate::domain::account_id::AccountId;
use shared::impl_domain_event;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AccountRemoved {
    pub metadata: EventMetadata,
    pub user_id: UserId,
    pub account_id: AccountId,
    pub account_type: &'static str,
}

impl AccountRemoved {
    pub fn new(user_id: UserId, account_id: AccountId, account_type: &'static str) -> Self {
        Self { metadata: EventMetadata::now(), user_id, account_id, account_type }
    }
}

impl_domain_event!(AccountRemoved, "account_removed", "ledger", "ledger");
