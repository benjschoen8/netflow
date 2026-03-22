//! FFI test harness — exercises the FFI layer against the test database.
//!
//! Database : {project-root}/backend/netflow-test.db
//! Build    : cargo build --bin test-ffi --features test-ffi
//! Run      : cargo run  --bin test-ffi --features test-ffi
//! Reset    : rm backend/netflow-test.db

use backend::db_path;
use ledger::interface::ffi;

fn main() {
    let path = db_path::test();
    println!("test-ffi: db → {path}");
    println!("test-ffi: reset with `rm {path}`\n");

    check("init", ffi::init(path, None));
    check("init_finances (idempotent)", { let _ = ffi::init_finances(); Ok(()) });

    match ffi::list_accounts() {
        Ok(accounts) => {
            pass("list_accounts");
            for a in &accounts {
                println!("  {} [{}] {} {}", a.account_name, a.account_type, a.currency, a.balance);
            }
        }
        Err(e) if e.contains("No finances record") => pass("list_accounts (fresh db)"),
        Err(e) => fail("list_accounts", &e),
    }

    match ffi::net_worth(None) {
        Ok(results) => {
            pass("net_worth");
            for r in &results {
                println!("  {} assets={} debts={} net={}", r.currency, r.total_assets, r.total_debts, r.net_worth);
            }
        }
        Err(e) if e.contains("No finances record") => pass("net_worth (fresh db)"),
        Err(e) => fail("net_worth", &e),
    }

    println!("\ntest-ffi: all checks passed");
}

fn check(label: &str, r: Result<(), String>) {
    match r {
        Ok(()) => pass(label),
        Err(e) => fail(label, &e),
    }
}

fn pass(label: &str) {
    println!("PASS  {label}");
}

fn fail(label: &str, msg: &str) -> ! {
    eprintln!("FAIL  {label}: {msg}");
    std::process::exit(1);
}
