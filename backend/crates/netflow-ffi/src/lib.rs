//! Thin binding surface for `flutter_rust_bridge`.
//!
//! All logic lives in `ledger::interface::ffi`.
//! This crate exists solely because Cargo requires a `[lib]` target with
//! `crate-type = ["cdylib", "staticlib"]` to produce the shared library
//! Flutter links against — you cannot set that on a `[[bin]]` target.
//!
//! Run `flutter_rust_bridge_codegen generate` from this crate's directory
//! to regenerate the Dart bindings in `frontend/lib/core/api/`.

use flutter_rust_bridge::frb;
pub use ledger::interface::ffi::*;

// Re-export public DTOs so frb codegen can see them.
pub use ledger::interface::ffi::{
    AccountSummary, NetWorthResult, LedgerEntry, Statement,
    CashAccountDetail, PhysicalWalletDetail, DigitalWalletDetail,
    HoldingDetail, InvestmentAccountDetail, LoanAccountDetail,
};

/// flutter_rust_bridge entry-point marker required by frb v2 codegen.
#[frb(init)]
pub fn frb_init() {
    flutter_rust_bridge::setup_default_user_code_handler();
}
