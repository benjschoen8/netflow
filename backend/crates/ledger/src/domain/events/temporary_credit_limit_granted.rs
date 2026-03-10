use serde::Serialize;
use chrono::NaiveDate;
use shared::domain::UserId;
use crate::domain::account_id::AccountId;
use crate::domain::money::Money;
use shared::domain::EventMetadata;
use shared::impl_domain_event;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TemporaryCreditLimitGranted {
    pub metadata: EventMetadata,
    pub user_id: UserId,
    pub account_id: AccountId,
    pub new_limit: Money,
    pub expires_on: NaiveDate,
}

impl TemporaryCreditLimitGranted {
    pub fn new(user_id: UserId, account_id: AccountId, new_limit: Money, expires_on: NaiveDate) -> Self {
        Self { metadata: EventMetadata::now(), user_id, account_id, new_limit, expires_on }
    }
}

impl_domain_event!(TemporaryCreditLimitGranted, "temporary_credit_limit_granted", "ledger", "ledger");
