use serde::Serialize;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::UserFinancesRepository;

pub struct ListAccountsQuery {
    pub owner_id: UserId,
}

/// Pure-primitive DTO — no domain types escape the application layer.
#[derive(Serialize)]
pub struct AccountSummary {
    pub account_id:   String,   // UUID as string
    pub account_name: String,
    pub account_type: &'static str,
    pub currency:     String,   // "USD" / "TWD"
    /// Positive = asset balance, negative sign indicates debt.
    pub balance:      String,
    pub is_debt:      bool,
    pub is_overdue:   bool,
}

pub async fn execute(
    repo: &dyn UserFinancesRepository,
    query: ListAccountsQuery,
) -> Result<Vec<AccountSummary>, LedgerError> {
    let finances = repo.load(query.owner_id).await?;

    let summaries = finances
        .accounts()
        .iter()
        .map(|a| {
            use crate::domain::balance::Balance;
            let (balance_str, is_debt) = match a.balance_summary() {
                Balance::Asset(m) => (m.amount().to_string(), false),
                Balance::Debt(l)  => (l.amount().to_string(), true),
            };
            AccountSummary {
                account_id:   a.account_id().to_string(),
                account_name: a.account_name_str().to_string(),
                account_type: a.account_type(),
                currency:     format!("{:?}", a.currency()),
                balance:      balance_str,
                is_debt,
                is_overdue:   a.is_overdue(),
            }
        })
        .collect();

    Ok(summaries)
}
