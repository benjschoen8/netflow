use serde::Serialize;
use shared::domain::UserId;
use crate::domain::account_id::AccountId;
use crate::domain::ticker::Ticker;
use shared::domain::EventMetadata;
use shared::impl_domain_event;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct HoldingRemoved {
    pub metadata: EventMetadata,
    pub user_id: UserId,
    pub account_id: AccountId,
    pub ticker: Ticker,
}

impl HoldingRemoved {
    pub fn new(user_id: UserId, account_id: AccountId, ticker: Ticker) -> Self {
        Self { metadata: EventMetadata::now(), user_id, account_id, ticker }
    }
}

impl_domain_event!(HoldingRemoved, "holding_removed", "ledger", "ledger");
