//! SQLite implementation of `LedgerUnitOfWork`.
//!
//! All writes inside `commit()` share a single `BEGIN IMMEDIATE / COMMIT`
//! transaction. If any write fails, sqlx rolls the whole transaction back
//! automatically when the `Transaction` is dropped without commit.

use async_trait::async_trait;
use rust_decimal::Decimal;
use shared::domain::UserId;
use sqlx::SqlitePool;
use std::str::FromStr;

use crate::application::error::LedgerError;
use crate::application::ports::ledger_unit_of_work::{LedgerUnitOfWork, WriteOperation};
use crate::domain::user_finances::UserFinances;
use super::account_row::{accounts_from_json, accounts_to_json};

pub struct SqliteLedgerUnitOfWork {
    pool: SqlitePool,
}

impl SqliteLedgerUnitOfWork {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LedgerUnitOfWork for SqliteLedgerUnitOfWork {
    // ── Load ─────────────────────────────────────────────────────────────────

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

    // ── Exists ────────────────────────────────────────────────────────────────

    async fn exists(&self, owner_id: UserId) -> Result<bool, LedgerError> {
        let row = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM user_finances WHERE owner_id = ?",
        )
        .bind(owner_id.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| LedgerError::Repository(e.to_string()))?;

        Ok(row.0 > 0)
    }

    // ── Commit ────────────────────────────────────────────────────────────────

    async fn commit(&self, op: WriteOperation<'_>) -> Result<(), LedgerError> {
        let mut tx = self.pool
            .begin()
            .await
            .map_err(|e| LedgerError::Repository(format!("begin tx: {e}")))?;

        // ── 1. Aggregate state ────────────────────────────────────────────────
        let owner_str = op.aggregate.owner_id().to_string();
        let json      = accounts_to_json(op.aggregate.accounts())
            .map_err(|e| LedgerError::Repository(e.to_string()))?;

        sqlx::query(
            r#"INSERT INTO user_finances (owner_id, accounts_json, version, updated_at)
               VALUES (?, ?, 1, datetime('now'))
               ON CONFLICT(owner_id) DO UPDATE SET
                   accounts_json = excluded.accounts_json,
                   version       = version + 1,
                   updated_at    = excluded.updated_at"#,
        )
        .bind(&owner_str)
        .bind(&json)
        .execute(&mut *tx)
        .await
        .map_err(|e| LedgerError::Repository(format!("save aggregate: {e}")))?;

        // ── 2. Ledger entries ─────────────────────────────────────────────────
        for entry in &op.entries {
            sqlx::query(
                "INSERT INTO ledger_entries
                 (id, account_id, entry_type, amount, currency, occurred_at, label, description)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(entry.id.to_string())
            .bind(entry.account_id.uuid().to_string())
            .bind(entry.entry_type.as_str())
            .bind(&entry.amount)
            .bind(&entry.currency)
            .bind(entry.occurred_at.to_rfc3339())
            .bind(&entry.label)
            .bind(&entry.description)
            .execute(&mut *tx)
            .await
            .map_err(|e| LedgerError::Repository(format!("save entry: {e}")))?;
        }

        // ── 3. New statement (close_statement only) ───────────────────────────
        if let Some(stmt) = &op.new_statement {
            sqlx::query(
                "INSERT INTO statements
                 (id, account_id, cycle_start, cycle_end,
                  statement_balance, minimum_payment, total_charged,
                  total_paid, is_settled, created_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(stmt.id.to_string())
            .bind(stmt.account_id.uuid().to_string())
            .bind(stmt.cycle_start.to_string())
            .bind(stmt.cycle_end.to_string())
            .bind(stmt.statement_balance.to_string())
            .bind(stmt.minimum_payment.map(|d| d.to_string()))
            .bind(stmt.total_charged.to_string())
            .bind(stmt.total_paid.to_string())
            .bind(stmt.is_settled)
            .bind(stmt.created_at.to_rfc3339())
            .execute(&mut *tx)
            .await
            .map_err(|e| LedgerError::Repository(format!("save statement: {e}")))?;
        }

        // ── 4. Credit existing statement (make_payment on credit card only) ───
        if let Some((stmt_id, payment)) = op.credit_statement {
            // Read current totals inside the same transaction for consistency
            let row = sqlx::query_as::<_, (String, String)>(
                "SELECT total_paid, statement_balance FROM statements WHERE id = ?",
            )
            .bind(stmt_id.to_string())
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| LedgerError::Repository(format!("read statement for credit: {e}")))?;

            if let Some((paid_str, balance_str)) = row {
                let new_paid   = Decimal::from_str(&paid_str).unwrap_or_default() + payment;
                let balance    = Decimal::from_str(&balance_str).unwrap_or_default();
                let is_settled = new_paid >= balance;

                sqlx::query(
                    "UPDATE statements SET total_paid = ?, is_settled = ? WHERE id = ?",
                )
                .bind(new_paid.to_string())
                .bind(is_settled)
                .bind(stmt_id.to_string())
                .execute(&mut *tx)
                .await
                .map_err(|e| LedgerError::Repository(format!("credit statement: {e}")))?;
            }
        }

        // ── Commit ────────────────────────────────────────────────────────────
        tx.commit()
            .await
            .map_err(|e| LedgerError::Repository(format!("commit tx: {e}")))?;

        Ok(())
    }
}
