//! netflow — personal finance API server.
//!
//! Bootstrap order:
//!   1. Parse config from env / args
//!   2. Open SQLite pool and run DDL migrations
//!   3. Build the repository (infrastructure)
//!   4. Build the Axum router (interface/http)
//!   5. Serve on the configured port

use std::sync::Arc;
use clap::Parser;
use ledger::{
    application::ports::UserFinancesRepository,
    infrastructure::{open_db, SqliteUserFinancesRepository},
    interface::{
        http::{AppState, build_router},
        resolve_user,
    },
};

/// netflow API server
#[derive(Parser, Debug)]
#[command(name = "netflow", version, about)]
struct Config {
    /// Path to the SQLite database file.
    #[arg(long, default_value = "netflow.db", env = "NETFLOW_DB")]
    db: String,

    /// Host and port to listen on.
    #[arg(long, default_value = "127.0.0.1:3000", env = "NETFLOW_ADDR")]
    addr: String,

    /// Fixed single-user UUID (until auth is added).
    #[arg(long, env = "NETFLOW_USER")]
    user: Option<uuid::Uuid>,
}

#[tokio::main]
async fn main() {
    let cfg = Config::parse();

    // ── Database ──────────────────────────────────────────────────────────────
    let pool = match open_db(&cfg.db).await {
        Ok(p)  => p,
        Err(e) => { eprintln!("error: {e}"); std::process::exit(1); }
    };

    // ── Repository ────────────────────────────────────────────────────────────
    let repo: Arc<dyn UserFinancesRepository> =
        Arc::new(SqliteUserFinancesRepository::new(pool));

    // ── Single-user identity ──────────────────────────────────────────────────
    let user_id = resolve_user(cfg.user);

    // ── Router ────────────────────────────────────────────────────────────────
    let state  = AppState::new(repo, user_id);
    let app    = build_router(state);

    // ── Serve ─────────────────────────────────────────────────────────────────
    let listener = tokio::net::TcpListener::bind(&cfg.addr)
        .await
        .unwrap_or_else(|e| { eprintln!("error: cannot bind {}: {e}", cfg.addr); std::process::exit(1); });

    println!("netflow API listening on http://{}", cfg.addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap_or_else(|e| { eprintln!("server error: {e}"); std::process::exit(1); });
}
