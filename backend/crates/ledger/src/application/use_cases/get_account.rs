//! Query: get a single account by ID.
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::UserFinancesRepository;
use crate::application::dto::AccountSummary;

pub struct GetAccountQuery {
    pub owner_id:   UserId,
    pub account_id: String,
}

pub async fn execute(
    repo: &dyn UserFinancesRepository,
    query: GetAccountQuery,
) -> Result<AccountSummary, LedgerError> {
    use crate::domain::balance::Balance;
    let finances = repo.load(query.owner_id).await?;

    let account = finances
        .accounts()
        .iter()
        .find(|a| a.account_id().to_string() == query.account_id)
        .ok_or(LedgerError::FinancesNotFound)?;

    let (balance, is_debt) = match account.balance_summary() {
        Balance::Asset(m) => (m.amount().to_string(), false),
        Balance::Debt(l)  => (l.amount().to_string(), true),
    };

    Ok(AccountSummary {
        account_id:   account.account_id().to_string(),
        account_name: account.account_name_str().to_string(),
        account_type: account.account_type(),
        currency:     format!("{:?}", account.currency()),
        balance,
        is_debt,
        is_overdue:   account.is_overdue(),
    })
}
