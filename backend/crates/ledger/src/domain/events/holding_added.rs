use serde::Serialize;
use rust_decimal::Decimal;
use shared::domain::UserId;
use crate::domain::account_id::AccountId;
use crate::domain::investment_type::InvestmentType;
use crate::domain::money::Money;
use crate::domain::ticker::Ticker;
use shared::domain::EventMetadata;
use shared::impl_domain_event;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct HoldingAdded {
    pub metadata: EventMetadata,
    pub user_id: UserId,
    pub account_id: AccountId,
    pub ticker: Ticker,
    pub investment_type: InvestmentType,
    pub quantity: Decimal,
    pub unit_price: Money,
}

impl HoldingAdded {
    pub fn new(user_id: UserId, account_id: AccountId, ticker: Ticker, investment_type: InvestmentType, quantity: Decimal, unit_price: Money) -> Self {
        Self { metadata: EventMetadata::now(), user_id, account_id, ticker, investment_type, quantity, unit_price }
    }
}

impl_domain_event!(HoldingAdded, "holding_added", "ledger", "ledger");
