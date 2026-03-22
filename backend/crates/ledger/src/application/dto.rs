//! Shared application-layer DTOs used by multiple query use cases.
//! These are pure-primitive output structs — no domain types.

use serde::Serialize;

/// Thin account summary returned by list and get-account queries.
#[derive(Serialize)]
pub struct AccountSummary {
    pub account_id:   String,
    pub account_name: String,
    pub account_type: &'static str,
    pub currency:     String,
    pub balance:      String,
    pub is_debt:      bool,
    pub is_overdue:   bool,
}

/// Net worth summary per currency.
#[derive(Serialize)]
pub struct NetWorthResult {
    pub currency:     String,
    pub total_assets: String,
    pub total_debts:  String,
    pub net_worth:    String,
    pub is_deficit:   bool,
}
