// ── Lifecycle ─────────────────────────────────────────────────────────────────
pub mod create_user_finances;

// ── Account management ────────────────────────────────────────────────────────
pub mod open_cash_account;
pub mod add_physical_wallet;
pub mod add_digital_wallet;
pub mod open_investment_account;
pub mod add_credit_card;
pub mod open_loan_account;
pub mod remove_account;
pub mod update_account_info;
pub mod transfer_funds;

// ── Asset mutations ───────────────────────────────────────────────────────────
pub mod deposit_funds;
pub mod withdraw_funds;

// ── Debt mutations ────────────────────────────────────────────────────────────
pub mod make_payment;
pub mod accrue_interest;

// ── Credit card specific ──────────────────────────────────────────────────────
pub mod charge_credit_card;
pub mod close_statement;
pub mod close_statement_with_record;
pub mod grant_temporary_limit;
pub mod revoke_temporary_limit;

// ── Investment specific ───────────────────────────────────────────────────────
pub mod add_holding;
pub mod remove_holding;
pub mod update_holding_price;

// ── Queries ───────────────────────────────────────────────────────────────────
pub mod list_accounts;
pub mod get_net_worth;
pub mod get_account;
