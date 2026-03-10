use serde::Serialize;
use shared::domain::UserId;
use crate::domain::account_id::AccountId;
use crate::domain::liability::Liability;
use shared::domain::EventMetadata;
use shared::impl_domain_event;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StatementClosed {
    pub metadata: EventMetadata,
    pub user_id: UserId,
    pub account_id: AccountId,
    pub statement_balance: Liability,
    pub minimum_payment: Option<Liability>,
}

impl StatementClosed {
    pub fn new(user_id: UserId, account_id: AccountId, statement_balance: Liability, minimum_payment: Option<Liability>) -> Self {
        Self { metadata: EventMetadata::now(), user_id, account_id, statement_balance, minimum_payment }
    }
}

impl_domain_event!(StatementClosed, "statement_closed", "ledger", "ledger");
