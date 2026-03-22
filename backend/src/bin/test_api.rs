//! Test HTTP API server — same as `api` but uses the test database.
//!
//! Database : {project-root}/backend/netflow-test.db  (created automatically)
//! Build    : cargo build --bin test-api --features test-api
//! Run      : cargo run  --bin test-api --features test-api
//! Reset db : rm backend/netflow-test.db

use std::sync::Arc;
use clap::Parser;
use ledger::{
    application::ports::{
        LedgerEntryRepository, LedgerUnitOfWork,
        StatementRepository, UserFinancesRepository,
    },
    infrastructure::{
        open_db,
        SqliteUserFinancesRepository, SqliteLedgerEntryRepository,
        SqliteStatementRepository, SqliteLedgerUnitOfWork,
    },
    interface::{http::{AppState, build_router}, resolve_user},
};
use backend::db_path;

#[derive(Parser, Debug)]
#[command(name = "netflow-test-api", about = "netflow HTTP API (test db)")]
struct Args {
    #[arg(long, env = "NETFLOW_TEST_DB")]
    db: Option<String>,

    /// Defaults to :3001 to avoid clashing with prod.
    #[arg(long, default_value = "127.0.0.1:3001", env = "NETFLOW_TEST_ADDR")]
    addr: String,

    #[arg(long, env = "NETFLOW_USER")]
    user: Option<uuid::Uuid>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let path = args.db.unwrap_or_else(db_path::test);

    println!("netflow-test-api: db  → {path}");
    println!("netflow-test-api: reset with `rm {path}`");

    let pool = open_db(&path).await
        .unwrap_or_else(|e| fatal(format!("db error: {e}")));

    let state = AppState::new(
        Arc::new(SqliteUserFinancesRepository::new(pool.clone())) as Arc<dyn UserFinancesRepository>,
        Arc::new(SqliteLedgerEntryRepository::new(pool.clone()))  as Arc<dyn LedgerEntryRepository>,
        Arc::new(SqliteStatementRepository::new(pool.clone()))    as Arc<dyn StatementRepository>,
        Arc::new(SqliteLedgerUnitOfWork::new(pool))               as Arc<dyn LedgerUnitOfWork>,
        resolve_user(args.user),
    );
    let app = build_router(state);

    let listener = tokio::net::TcpListener::bind(&args.addr).await
        .unwrap_or_else(|e| fatal(format!("bind {}: {e}", args.addr)));

    println!("netflow-test-api: http://{}", args.addr);
    axum::serve(listener, app).await
        .unwrap_or_else(|e| fatal(format!("server: {e}")));
}

fn fatal(msg: String) -> ! {
    eprintln!("error: {msg}");
    std::process::exit(1);
}
