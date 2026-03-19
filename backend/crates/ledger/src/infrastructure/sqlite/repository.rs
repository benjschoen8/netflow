use async_trait::async_trait;
use sqlx::SqlitePool;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::UserFinancesRepository;
use crate::domain::user_finances::UserFinances;
use super::account_row::{accounts_to_json, accounts_from_json};

pub struct SqliteUserFinancesRepository {
    pool: SqlitePool,
}

impl SqliteUserFinancesRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserFinancesRepository for SqliteUserFinancesRepository {
    async fn load(&self, owner_id: UserId) -> Result<UserFinances, LedgerError> {
        let id_str = owner_id.to_string();

        let row = sqlx::query_as::<_, (String,)>(
            "SELECT accounts_json FROM user_finances WHERE owner_id = ?",
        )
        .bind(&id_str)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| LedgerError::Repository(e.to_string()))?
        .ok_or(LedgerError::FinancesNotFound)?;

        let accounts = accounts_from_json(&row.0)
            .map_err(|e| LedgerError::Repository(e.to_string()))?;

        Ok(UserFinances::restore(owner_id, accounts))
    }

    async fn save(&self, aggregate: &UserFinances) -> Result<(), LedgerError> {
        let id_str = aggregate.owner_id().to_string();
        let json   = accounts_to_json(aggregate.accounts())
            .map_err(|e| LedgerError::Repository(e.to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO user_finances (owner_id, accounts_json, version, updated_at)
            VALUES (?, ?, 1, datetime('now'))
            ON CONFLICT(owner_id) DO UPDATE SET
                accounts_json = excluded.accounts_json,
                version       = version + 1,
                updated_at    = excluded.updated_at
            "#,
        )
        .bind(&id_str)
        .bind(&json)
        .execute(&self.pool)
        .await
        .map_err(|e| LedgerError::Repository(e.to_string()))?;

        Ok(())
    }

    async fn exists(&self, owner_id: UserId) -> Result<bool, LedgerError> {
        let id_str = owner_id.to_string();

        let row = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM user_finances WHERE owner_id = ?",
        )
        .bind(&id_str)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| LedgerError::Repository(e.to_string()))?;

        Ok(row.0 > 0)
    }
}
