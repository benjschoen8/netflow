use serde::Serialize;
use shared::domain::UserId;
use shared::domain::EventMetadata;
use shared::impl_domain_event;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UserFinancesCreated {
    pub metadata: EventMetadata,
    pub user_id: UserId,
}

impl UserFinancesCreated {
    pub fn new(user_id: UserId) -> Self {
        Self { metadata: EventMetadata::now(), user_id }
    }
}

impl_domain_event!(UserFinancesCreated, "user_finances_created", "ledger", "ledger");
