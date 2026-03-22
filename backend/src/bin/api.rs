//! Production HTTP API server.
//!
//! Database : {project-root}/data/netflow.db  (created automatically)
//! Build    : cargo build --bin api --features api --release
//! Run      : cargo run  --bin api --features api [-- --addr 0.0.0.0:3000]

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
#[command(name = "netflow-api", about = "netflow HTTP API (prod)")]
struct Args {
    /// Override the default db path.
    #[arg(long, env = "NETFLOW_DB")]
    db: Option<String>,

    #[arg(long, default_value = "127.0.0.1:3000", env = "NETFLOW_ADDR")]
    addr: String,

    #[arg(long, env = "NETFLOW_USER")]
    user: Option<uuid::Uuid>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let path = args.db.unwrap_or_else(db_path::prod);
    ensure_parent(&path);

    println!("netflow-api: db  → {path}");

    let pool = open_db(&path).await
        .unwrap_or_else(|e| fatal(format!("db error: {e}")));

    let state = build_state(pool, args.user);
    let app   = build_router(state);

    let listener = tokio::net::TcpListener::bind(&args.addr).await
        .unwrap_or_else(|e| fatal(format!("bind {}: {e}", args.addr)));

    println!("netflow-api: http://{}",  args.addr);
    axum::serve(listener, app).await
        .unwrap_or_else(|e| fatal(format!("server: {e}")));
}

fn build_state(pool: sqlx::SqlitePool, user: Option<uuid::Uuid>) -> AppState {
    AppState::new(
        Arc::new(SqliteUserFinancesRepository::new(pool.clone())) as Arc<dyn UserFinancesRepository>,
        Arc::new(SqliteLedgerEntryRepository::new(pool.clone()))  as Arc<dyn LedgerEntryRepository>,
        Arc::new(SqliteStatementRepository::new(pool.clone()))    as Arc<dyn StatementRepository>,
        Arc::new(SqliteLedgerUnitOfWork::new(pool))               as Arc<dyn LedgerUnitOfWork>,
        resolve_user(user),
    )
}

fn ensure_parent(path: &str) {
    if let Some(p) = std::path::Path::new(path).parent() {
        if !p.as_os_str().is_empty() && !p.exists() {
            std::fs::create_dir_all(p)
                .unwrap_or_else(|e| fatal(format!("mkdir {}: {e}", p.display())));
        }
    }
}

fn fatal(msg: String) -> ! {
    eprintln!("error: {msg}");
    std::process::exit(1);
}
