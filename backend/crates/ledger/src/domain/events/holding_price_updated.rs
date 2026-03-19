use serde::Serialize;
use shared::domain::UserId;
use shared::domain::EventMetadata;
use shared::impl_domain_event;
use crate::domain::account_id::AccountId;
use crate::domain::money::Money;
use crate::domain::ticker::Ticker;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct HoldingPriceUpdated {
    pub metadata: EventMetadata,
    pub user_id: UserId,
    pub account_id: AccountId,
    pub ticker: Ticker,
    pub new_price: Money,
}

impl HoldingPriceUpdated {
    pub fn new(
        user_id: UserId,
        account_id: AccountId,
        ticker: Ticker,
        new_price: Money,
    ) -> Self {
        Self { metadata: EventMetadata::now(), user_id, account_id, ticker, new_price }
    }
}

impl_domain_event!(HoldingPriceUpdated, "holding_price_updated", "ledger", "ledger");
