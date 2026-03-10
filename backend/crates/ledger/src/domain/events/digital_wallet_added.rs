use serde::Serialize;
use shared::domain::UserId;
use crate::domain::account_id::AccountId;
use crate::domain::currency::Currency;
use crate::domain::digital_wallet_provider::DigitalWalletProvider;
use shared::domain::EventMetadata;
use shared::impl_domain_event;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DigitalWalletAdded {
    pub metadata: EventMetadata,
    pub user_id: UserId,
    pub account_id: AccountId,
    pub account_name: String,
    pub provider: DigitalWalletProvider,
    pub currency: Currency,
}

impl DigitalWalletAdded {
    pub fn new(user_id: UserId, account_id: AccountId, account_name: String, provider: DigitalWalletProvider, currency: Currency) -> Self {
        Self { metadata: EventMetadata::now(), user_id, account_id, account_name, provider, currency }
    }
}

impl_domain_event!(DigitalWalletAdded, "digital_wallet_added", "ledger", "ledger");
