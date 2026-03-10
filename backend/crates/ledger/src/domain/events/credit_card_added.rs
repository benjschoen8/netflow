use serde::Serialize;
use shared::domain::UserId;
use crate::domain::account_id::AccountId;
use crate::domain::card_network::CardNetwork;
use crate::domain::currency::Currency;
use crate::domain::money::Money;
use shared::domain::EventMetadata;
use shared::impl_domain_event;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CreditCardAdded {
    pub metadata: EventMetadata,
    pub user_id: UserId,
    pub account_id: AccountId,
    pub account_name: String,
    pub network: CardNetwork,
    pub currency: Currency,
    pub credit_limit: Money,
}

impl CreditCardAdded {
    pub fn new(user_id: UserId, account_id: AccountId, account_name: String, network: CardNetwork, currency: Currency, credit_limit: Money) -> Self {
        Self { metadata: EventMetadata::now(), user_id, account_id, account_name, network, currency, credit_limit }
    }
}

impl_domain_event!(CreditCardAdded, "credit_card_added", "ledger", "ledger");
