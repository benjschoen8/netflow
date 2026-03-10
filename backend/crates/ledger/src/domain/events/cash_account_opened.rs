use serde::Serialize;
use shared::domain::UserId;
use crate::domain::account_id::AccountId;
use crate::domain::currency::Currency;
use shared::domain::EventMetadata;
use shared::impl_domain_event;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CashAccountOpened {
    pub metadata: EventMetadata,
    pub user_id: UserId,
    pub account_id: AccountId,
    pub account_name: String,
    pub currency: Currency,
}

impl CashAccountOpened {
    pub fn new(user_id: UserId, account_id: AccountId, account_name: String, currency: Currency) -> Self {
        Self { metadata: EventMetadata::now(), user_id, account_id, account_name, currency }
    }
}

impl_domain_event!(CashAccountOpened, "cash_account_opened", "ledger", "ledger");
