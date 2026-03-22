//! No-op stub implementations used by the CLI interface.
//!
//! The CLI does not maintain billing statements, so make_payment passes
//! this stub rather than a real SqliteStatementRepository.

use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::application::error::LedgerError;
use crate::application::ports::StatementRepository;
use crate::domain::account_id::AccountId;
use crate::domain::statement::Statement;

pub struct NoOpStatementRepository;

#[async_trait]
impl StatementRepository for NoOpStatementRepository {
    async fn save(&self, _: &Statement) -> Result<(), LedgerError> { Ok(()) }

    async fn list_for_account(&self, _: AccountId) -> Result<Vec<Statement>, LedgerError> {
        Ok(vec![])
    }

    async fn find(&self, _: Uuid) -> Result<Option<Statement>, LedgerError> { Ok(None) }

    async fn find_latest_unsettled(&self, _: AccountId) -> Result<Option<Statement>, LedgerError> {
        Ok(None)
    }

    async fn find_latest(&self, _: AccountId) -> Result<Option<Statement>, LedgerError> {
        Ok(None)
    }

    async fn apply_payment(&self, _: Uuid, _: Decimal) -> Result<(), LedgerError> { Ok(()) }
}
