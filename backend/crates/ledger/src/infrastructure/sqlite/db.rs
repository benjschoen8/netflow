use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::str::FromStr;
use shared::domain::SharedError;

/// Open (and create if needed) the SQLite database at `path`,
/// then run DDL migrations inline.
///
/// We run the migration SQL directly rather than using the sqlx migrate!()
/// macro so that the binary has no dependency on an external migrations
/// directory at runtime, and requires no DATABASE_URL at compile time.
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

/// Execute all schema migrations against the open pool.
/// Each statement is idempotent (IF NOT EXISTS).
async fn run_migrations(pool: &SqlitePool) -> Result<(), SharedError> {
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

    Ok(())
}
