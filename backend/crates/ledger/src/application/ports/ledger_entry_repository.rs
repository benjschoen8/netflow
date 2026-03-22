use async_trait::async_trait;
use uuid::Uuid;

use crate::application::error::LedgerError;
use crate::domain::account_id::AccountId;
use crate::domain::ledger_entry::LedgerEntry;

/// Port for persisting and reading ledger entries.
/// Implemented by the SQLite infrastructure layer.
#[async_trait]
pub trait LedgerEntryRepository: Send + Sync {
    /// Persist a new entry. Called after every successful transaction.
    async fn save(&self, entry: &LedgerEntry) -> Result<(), LedgerError>;

    /// All entries for an account, newest first.
    async fn list_for_account(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<LedgerEntry>, LedgerError>;

    /// Update only the label and description of an existing entry.
    async fn update_annotation(
        &self,
        entry_id:    Uuid,
        label:       Option<String>,
        description: Option<String>,
    ) -> Result<(), LedgerError>;

    /// Fetch a single entry by ID.
    async fn find(
        &self,
        entry_id: Uuid,
    ) -> Result<Option<LedgerEntry>, LedgerError>;
}
