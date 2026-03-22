use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::interface::http::app_state::AppState;
use crate::interface::http::handlers::{
    account_detail, accounts, card_info, entries, holdings, statements, transactions,
};

pub fn build(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // ── Lifecycle ─────────────────────────────────────────────────────────
        .route("/init", post(accounts::init))

        // ── Account list + creation ────────────────────────────────────────────
        .route("/accounts",                get(accounts::list))
        .route("/accounts/cash",           post(accounts::open_cash))
        .route("/accounts/wallet",         post(accounts::add_wallet))
        .route("/accounts/digital-wallet", post(accounts::add_digital_wallet_handler))
        .route("/accounts/investment",     post(accounts::open_investment))
        .route("/accounts/credit-card",    post(accounts::add_credit_card_handler))
        .route("/accounts/loan",           post(accounts::open_loan))
        .route("/accounts/{id}",           delete(accounts::remove))
        .route("/accounts/{id}",           patch(accounts::update_info))

        // ── Rich per-type detail ───────────────────────────────────────────────
        .route("/accounts/{id}/detail/cash",            get(account_detail::cash_detail))
        .route("/accounts/{id}/detail/wallet",          get(account_detail::wallet_detail))
        .route("/accounts/{id}/detail/digital-wallet",  get(account_detail::digital_wallet_detail))
        .route("/accounts/{id}/detail/investment",      get(account_detail::investment_detail))
        .route("/accounts/{id}/detail/loan",            get(account_detail::loan_detail))
        .route("/accounts/{id}/detail/credit-card",     get(card_info::get_card_info))

        // ── Transactions ───────────────────────────────────────────────────────
        .route("/accounts/{id}/deposit",      post(transactions::deposit))
        .route("/accounts/{id}/withdraw",     post(transactions::withdraw))
        .route("/accounts/{id}/transfer",     post(accounts::transfer))
        .route("/accounts/{id}/charge",       post(transactions::charge))
        .route("/accounts/{id}/pay",          post(transactions::pay))
        .route("/accounts/{id}/statement",    post(transactions::close_statement_handler))
        .route("/accounts/{id}/grant-limit",  post(transactions::grant_limit))
        .route("/accounts/{id}/revoke-limit", post(transactions::revoke_limit))
        .route("/accounts/{id}/interest",     post(transactions::accrue_interest_handler))

        // ── Ledger entries ─────────────────────────────────────────────────────
        .route("/accounts/{id}/entries",      get(entries::list))
        .route("/entries/{entry_id}",         patch(entries::update_annotation))

        // ── Statements (credit card billing cycles) ────────────────────────────
        .route("/accounts/{id}/statements",                          get(statements::list))
        .route("/accounts/{id}/statements/{stmt_id}/entries",        get(statements::statement_entries))

        // ── Holdings ──────────────────────────────────────────────────────────
        .route("/accounts/{id}/holdings",                      post(holdings::add))
        .route("/accounts/{id}/holdings/{ticker}",             delete(holdings::remove))
        .route("/accounts/{id}/holdings/{ticker}/price",       patch(holdings::update_price))

        // ── Summary ────────────────────────────────────────────────────────────
        .route("/net-worth", get(accounts::net_worth))

        .layer(cors)
        .with_state(state)
}
