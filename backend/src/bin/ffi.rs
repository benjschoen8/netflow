//! Desktop runner for the FFI layer against the prod database.
//! Initialises the FFI state, runs a quick sanity check, then exits.
//! Useful for verifying the compiled library works before shipping to mobile.
//!
//! Build : cargo build --bin ffi --features ffi-bin --release
//! Run   : cargo run  --bin ffi --features ffi-bin

use backend::db_path;
use ledger::interface::ffi;

fn main() {
    let path = db_path::prod();
    println!("ffi: db → {path}");

    ffi::init(path, None).unwrap_or_else(|e| {
        eprintln!("error: init failed: {e}");
        std::process::exit(1);
    });

    match ffi::list_accounts() {
        Ok(accounts) => {
            println!("ffi: {} account(s)", accounts.len());
            for a in &accounts {
                println!("  {} [{}] {} {}", a.account_name, a.account_type, a.currency, a.balance);
            }
        }
        Err(e) if e.contains("No finances record") => {
            println!("ffi: fresh db — call ffi::init_finances() to initialise");
        }
        Err(e) => {
            eprintln!("error: list_accounts: {e}");
            std::process::exit(1);
        }
    }

    println!("ffi: ok");
}
