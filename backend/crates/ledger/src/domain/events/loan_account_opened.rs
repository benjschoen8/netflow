use serde::Serialize;
use shared::domain::UserId;
use crate::domain::account_id::AccountId;
use crate::domain::liability::Liability;
use shared::domain::EventMetadata;
use shared::impl_domain_event;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LoanAccountOpened {
    pub metadata: EventMetadata,
    pub user_id: UserId,
    pub account_id: AccountId,
    pub account_name: String,
    pub principal: Liability,
    pub creditor: String,
}

impl LoanAccountOpened {
    pub fn new(user_id: UserId, account_id: AccountId, account_name: String, principal: Liability, creditor: String) -> Self {
        Self { metadata: EventMetadata::now(), user_id, account_id, account_name, principal, creditor }
    }
}

impl_domain_event!(LoanAccountOpened, "loan_account_opened", "ledger", "ledger");
