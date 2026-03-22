use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::str::FromStr;
use shared::domain::SharedError;

pub async fn open(path: &str) -> Result<SqlitePool, SharedError> {
    let opts = SqliteConnectOptions::from_str(path)
        .map_err(|e| SharedError::Database(e.to_string()))?
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(opts)
        .await
        .map_err(|e| SharedError::Database(e.to_string()))?;

    run_migrations(&pool).await?;
    Ok(pool)
}

async fn run_migrations(pool: &SqlitePool) -> Result<(), SharedError> {
    // user_finances aggregate store
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_finances (
            owner_id      TEXT    NOT NULL PRIMARY KEY,
            accounts_json TEXT    NOT NULL DEFAULT '[]',
            version       INTEGER NOT NULL DEFAULT 0,
            created_at    TEXT    NOT NULL DEFAULT (datetime('now')),
            updated_at    TEXT    NOT NULL DEFAULT (datetime('now'))
        )",
    )
    .execute(pool)
    .await
    .map_err(|e| SharedError::Database(e.to_string()))?;

    // ledger_entries — one row per transaction event
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS ledger_entries (
            id          TEXT    NOT NULL PRIMARY KEY,
            account_id  TEXT    NOT NULL,
            entry_type  TEXT    NOT NULL,
            amount      TEXT    NOT NULL,
            currency    TEXT    NOT NULL,
            occurred_at TEXT    NOT NULL,
            label       TEXT,
            description TEXT
        )",
    )
    .execute(pool)
    .await
    .map_err(|e| SharedError::Database(e.to_string()))?;

    // Index for fast per-account queries
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_ledger_entries_account_id
         ON ledger_entries (account_id, occurred_at DESC)",
    )
    .execute(pool)
    .await
    .map_err(|e| SharedError::Database(e.to_string()))?;


    // statements — one row per closed billing cycle
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS statements (
            id                TEXT    NOT NULL PRIMARY KEY,
            account_id        TEXT    NOT NULL,
            cycle_start       TEXT    NOT NULL,
            cycle_end         TEXT    NOT NULL,
            statement_balance TEXT    NOT NULL,
            minimum_payment   TEXT,
            total_charged     TEXT    NOT NULL DEFAULT '0',
            total_paid        TEXT    NOT NULL DEFAULT '0',
            is_settled        INTEGER NOT NULL DEFAULT 0,
            created_at        TEXT    NOT NULL
        )",
    )
    .execute(pool)
    .await
    .map_err(|e| SharedError::Database(e.to_string()))?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_statements_account_id
         ON statements (account_id, cycle_end DESC)",
    )
    .execute(pool)
    .await
    .map_err(|e| SharedError::Database(e.to_string()))?;

    Ok(())
}
