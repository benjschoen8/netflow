use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use sqlx::SqlitePool;
use std::str::FromStr;
use uuid::Uuid;

use crate::application::error::LedgerError;
use crate::application::ports::StatementRepository;
use crate::domain::account_id::AccountId;
use crate::domain::statement::Statement;

pub struct SqliteStatementRepository {
    pool: SqlitePool,
}

impl SqliteStatementRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StatementRepository for SqliteStatementRepository {
    async fn save(&self, s: &Statement) -> Result<(), LedgerError> {
        sqlx::query(
            "INSERT INTO statements
             (id, account_id, cycle_start, cycle_end,
              statement_balance, minimum_payment, total_charged,
              total_paid, is_settled, created_at)
             VALUES (?,?,?,?,?,?,?,?,?,?)",
        )
        .bind(s.id.to_string())
        .bind(s.account_id.uuid().to_string())
        .bind(s.cycle_start.to_string())
        .bind(s.cycle_end.to_string())
        .bind(s.statement_balance.to_string())
        .bind(s.minimum_payment.map(|d| d.to_string()))
        .bind(s.total_charged.to_string())
        .bind(s.total_paid.to_string())
        .bind(s.is_settled)
        .bind(s.created_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| LedgerError::Repository(e.to_string()))?;
        Ok(())
    }

    async fn list_for_account(
        &self,
        account_id: AccountId,
    ) -> Result<Vec<Statement>, LedgerError> {
        let rows = sqlx::query_as::<_, StatementRow>(
            "SELECT * FROM statements
             WHERE account_id = ?
             ORDER BY cycle_end DESC",
        )
        .bind(account_id.uuid().to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| LedgerError::Repository(e.to_string()))?;

        rows.into_iter().map(row_to_statement).collect()
    }

    async fn find(&self, id: Uuid) -> Result<Option<Statement>, LedgerError> {
        let row = sqlx::query_as::<_, StatementRow>(
            "SELECT * FROM statements WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| LedgerError::Repository(e.to_string()))?;

        row.map(row_to_statement).transpose()
    }

    async fn find_latest_unsettled(
        &self,
        account_id: AccountId,
    ) -> Result<Option<Statement>, LedgerError> {
        let row = sqlx::query_as::<_, StatementRow>(
            "SELECT * FROM statements
             WHERE account_id = ? AND is_settled = 0
             ORDER BY cycle_end DESC
             LIMIT 1",
        )
        .bind(account_id.uuid().to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| LedgerError::Repository(e.to_string()))?;

        row.map(row_to_statement).transpose()
    }

    async fn find_latest(
        &self,
        account_id: AccountId,
    ) -> Result<Option<Statement>, LedgerError> {
        let row = sqlx::query_as::<_, StatementRow>(
            "SELECT * FROM statements
             WHERE account_id = ?
             ORDER BY cycle_end DESC
             LIMIT 1",
        )
        .bind(account_id.uuid().to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| LedgerError::Repository(e.to_string()))?;

        row.map(row_to_statement).transpose()
    }

    async fn apply_payment(
        &self,
        statement_id: Uuid,
        amount:       Decimal,
    ) -> Result<(), LedgerError> {
        // Fetch current totals
        let row = sqlx::query_as::<_, (String, String)>(
            "SELECT total_paid, statement_balance FROM statements WHERE id = ?",
        )
        .bind(statement_id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| LedgerError::Repository(e.to_string()))?;

        let (paid_str, balance_str) = match row {
            None => return Ok(()),
            Some(r) => r,
        };

        let new_paid    = Decimal::from_str(&paid_str).unwrap_or_default() + amount;
        let balance     = Decimal::from_str(&balance_str).unwrap_or_default();
        let is_settled  = new_paid >= balance;

        sqlx::query(
            "UPDATE statements SET total_paid = ?, is_settled = ? WHERE id = ?",
        )
        .bind(new_paid.to_string())
        .bind(is_settled)
        .bind(statement_id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| LedgerError::Repository(e.to_string()))?;

        Ok(())
    }
}

// ── Row type ──────────────────────────────────────────────────────────────────

#[derive(sqlx::FromRow)]
struct StatementRow {
    id:                String,
    account_id:        String,
    cycle_start:       String,
    cycle_end:         String,
    statement_balance: String,
    minimum_payment:   Option<String>,
    total_charged:     String,
    total_paid:        String,
    is_settled:        bool,
    created_at:        String,
}

fn row_to_statement(r: StatementRow) -> Result<Statement, LedgerError> {
    let parse_dec = |s: &str| {
        Decimal::from_str(s)
            .map_err(|e| LedgerError::Repository(format!("bad decimal '{s}': {e}")))
    };
    let parse_date = |s: &str| {
        NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map_err(|e| LedgerError::Repository(format!("bad date '{s}': {e}")))
    };

    Ok(Statement {
        id: r.id.parse::<Uuid>()
            .map_err(|e| LedgerError::Repository(e.to_string()))?,
        account_id: AccountId::restore(
            r.account_id.parse::<Uuid>()
                .map_err(|e| LedgerError::Repository(e.to_string()))?,
        ).map_err(|e| LedgerError::Repository(e.to_string()))?,
        cycle_start:       parse_date(&r.cycle_start)?,
        cycle_end:         parse_date(&r.cycle_end)?,
        statement_balance: parse_dec(&r.statement_balance)?,
        minimum_payment:   r.minimum_payment.as_deref().map(parse_dec).transpose()?,
        total_charged:     parse_dec(&r.total_charged)?,
        total_paid:        parse_dec(&r.total_paid)?,
        is_settled:        r.is_settled,
        created_at:        DateTime::parse_from_rfc3339(&r.created_at)
            .map_err(|e| LedgerError::Repository(format!("bad created_at: {e}")))?
            .with_timezone(&Utc),
    })
}
