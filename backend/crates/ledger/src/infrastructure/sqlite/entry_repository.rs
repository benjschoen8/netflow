use async_trait::async_trait;
use chrono::DateTime;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::application::error::LedgerError;
use crate::application::ports::LedgerEntryRepository;
use crate::domain::account_id::AccountId;
use crate::domain::ledger_entry::{EntryType, LedgerEntry};

pub struct SqliteLedgerEntryRepository {
    pool: SqlitePool,
}

impl SqliteLedgerEntryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LedgerEntryRepository for SqliteLedgerEntryRepository {
    async fn save(&self, entry: &LedgerEntry) -> Result<(), LedgerError> {
        let id          = entry.id.to_string();
        let account_id  = entry.account_id.uuid().to_string();
        let entry_type  = entry.entry_type.as_str();
        let occurred_at = entry.occurred_at.to_rfc3339();

        sqlx::query(
            "INSERT INTO ledger_entries
             (id, account_id, entry_type, amount, currency, occurred_at, label, description)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&account_id)
        .bind(entry_type)
        .bind(&entry.amount)
        .bind(&entry.currency)
        .bind(&occurred_at)
        .bind(&entry.label)
        .bind(&entry.description)
        .execute(&self.pool)
        .await
        .map_err(|e| LedgerError::Repository(e.to_string()))?;

        Ok(())
    }

    async fn list_for_account(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<LedgerEntry>, LedgerError> {
        let id_str = account_id.uuid().to_string();

        let rows = sqlx::query_as::<_, EntryRow>(
            "SELECT id, account_id, entry_type, amount, currency,
                    occurred_at, label, description
             FROM ledger_entries
             WHERE account_id = ?
             ORDER BY occurred_at DESC",
        )
        .bind(&id_str)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| LedgerError::Repository(e.to_string()))?;

        rows.into_iter().map(row_to_entry).collect()
    }

    async fn update_annotation(
        &self,
        entry_id:    Uuid,
        label:       Option<String>,
        description: Option<String>,
    ) -> Result<(), LedgerError> {
        let id_str = entry_id.to_string();

        sqlx::query(
            "UPDATE ledger_entries
             SET label = ?, description = ?
             WHERE id = ?",
        )
        .bind(&label)
        .bind(&description)
        .bind(&id_str)
        .execute(&self.pool)
        .await
        .map_err(|e| LedgerError::Repository(e.to_string()))?;

        Ok(())
    }

    async fn find(&self, entry_id: Uuid) -> Result<Option<LedgerEntry>, LedgerError> {
        let id_str = entry_id.to_string();

        let row = sqlx::query_as::<_, EntryRow>(
            "SELECT id, account_id, entry_type, amount, currency,
                    occurred_at, label, description
             FROM ledger_entries WHERE id = ?",
        )
        .bind(&id_str)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| LedgerError::Repository(e.to_string()))?;

        row.map(row_to_entry).transpose()
    }
}

// ── Internal row type ─────────────────────────────────────────────────────────

#[derive(sqlx::FromRow)]
struct EntryRow {
    id:          String,
    account_id:  String,
    entry_type:  String,
    amount:      String,
    currency:    String,
    occurred_at: String,
    label:       Option<String>,
    description: Option<String>,
}

fn row_to_entry(r: EntryRow) -> Result<LedgerEntry, LedgerError> {
    let id = r.id.parse::<Uuid>()
        .map_err(|e| LedgerError::Repository(format!("bad entry id: {e}")))?;

    let account_uuid = r.account_id.parse::<Uuid>()
        .map_err(|e| LedgerError::Repository(format!("bad account_id: {e}")))?;

    let account_id = AccountId::restore(account_uuid)
        .map_err(|e| LedgerError::Repository(e.to_string()))?;

    let entry_type = EntryType::from_str(&r.entry_type)
        .ok_or_else(|| LedgerError::Repository(
            format!("unknown entry_type: {}", r.entry_type)
        ))?;

    let occurred_at = DateTime::parse_from_rfc3339(&r.occurred_at)
        .map_err(|e| LedgerError::Repository(format!("bad occurred_at: {e}")))?
        .with_timezone(&chrono::Utc);

    Ok(LedgerEntry {
        id,
        account_id,
        entry_type,
        amount: r.amount,
        currency: r.currency,
        occurred_at,
        label: r.label,
        description: r.description,
    })
}
