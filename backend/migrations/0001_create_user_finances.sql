-- Single-table JSON persistence for UserFinances aggregate.
-- All accounts are stored as a JSON array in `accounts_json`.
-- This keeps the schema minimal for a CLI tool while remaining
-- easy to migrate to a normalised schema in the future.
CREATE TABLE IF NOT EXISTS user_finances (
    owner_id      TEXT    NOT NULL PRIMARY KEY,
    accounts_json TEXT    NOT NULL DEFAULT '[]',
    version       INTEGER NOT NULL DEFAULT 0,
    created_at    TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at    TEXT    NOT NULL DEFAULT (datetime('now'))
);
