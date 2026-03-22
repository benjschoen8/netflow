use async_trait::async_trait;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::application::error::LedgerError;
use crate::domain::account_id::AccountId;
use crate::domain::statement::Statement;

#[async_trait]
pub trait StatementRepository: Send + Sync {
    /// Persist a newly closed statement.
    async fn save(&self, statement: &Statement) -> Result<(), LedgerError>;

    /// All statements for an account, newest first.
    async fn list_for_account(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<Statement>, LedgerError>;

    /// Single statement by ID.
    async fn find(
        &self,
        statement_id: Uuid,
    ) -> Result<Option<Statement>, LedgerError>;

    /// The most recent unsettled statement for an account.
    /// Used when a payment comes in to know which bill to credit.
    async fn find_latest_unsettled(
        &self,
        account_id: AccountId,
    ) -> Result<Option<Statement>, LedgerError>;

    /// The most recently closed statement — used to calculate next cycle_start.
    async fn find_latest(
        &self,
        account_id: AccountId,
    ) -> Result<Option<Statement>, LedgerError>;

    /// Apply a payment: increment total_paid, update is_settled.
    async fn apply_payment(
        &self,
        statement_id: Uuid,
        amount:       Decimal,
    ) -> Result<(), LedgerError>;
}
