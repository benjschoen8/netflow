use std::sync::Arc;
use shared::domain::UserId;
use crate::application::ports::{
    LedgerEntryRepository,
    LedgerUnitOfWork,
    StatementRepository,
    UserFinancesRepository,
};

#[derive(Clone)]
pub struct AppState {
    /// Read-only: used by query use cases (list_accounts, get_net_worth, get_account).
    pub repo: Arc<dyn UserFinancesRepository>,

    /// Read-only: used to list entries and statement entries for display.
    pub entry_repo: Arc<dyn LedgerEntryRepository>,

    /// Read-only: used to list statements and find open statements.
    pub statement_repo: Arc<dyn StatementRepository>,

    /// Atomic write: used by all mutation use cases.
    /// Wraps aggregate save + entry inserts + statement writes in one transaction.
    pub uow: Arc<dyn LedgerUnitOfWork>,

    pub user_id: UserId,
}

impl AppState {
    pub fn new(
        repo:           Arc<dyn UserFinancesRepository>,
        entry_repo:     Arc<dyn LedgerEntryRepository>,
        statement_repo: Arc<dyn StatementRepository>,
        uow:            Arc<dyn LedgerUnitOfWork>,
        user_id:        UserId,
    ) -> Self {
        Self { repo, entry_repo, statement_repo, uow, user_id }
    }
}
