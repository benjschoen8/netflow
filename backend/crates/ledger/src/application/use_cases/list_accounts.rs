use shared::domain::UserId;
use crate::application::dto::AccountSummary;
use crate::application::error::LedgerError;
use crate::application::ports::UserFinancesRepository;
use crate::domain::balance::Balance;
use crate::domain::financial_account::FinancialAccount;

pub struct ListAccountsQuery {
    pub owner_id: UserId,
}

pub async fn execute(
    repo: &dyn UserFinancesRepository,
    query: ListAccountsQuery,
) -> Result<Vec<AccountSummary>, LedgerError> {
    let finances = repo.load(query.owner_id).await?;

    finances
        .accounts()
        .iter()
        .map(account_to_summary)
        .collect()
}

/// Build a summary for a single account.
/// Investment accounts report total_value (cash + holdings),
/// not just cash balance, so the dashboard tile reflects the full position.
pub(crate) fn account_to_summary(a: &FinancialAccount) -> Result<AccountSummary, LedgerError> {
    let (balance_str, is_debt) = match a {
        FinancialAccount::Investment(inv) => {
            let total = inv.total_value()?;
            (total.amount().to_string(), false)
        }
        other => match other.balance_summary() {
            Balance::Asset(m) => (m.amount().to_string(), false),
            Balance::Debt(l)  => (l.amount().to_string(), true),
        },
    };

    Ok(AccountSummary {
        account_id:   a.account_id().to_string(),
        account_name: a.account_name_str().to_string(),
        account_type: a.account_type(),
        currency:     format!("{:?}", a.currency()),
        balance:      balance_str,
        is_debt,
        is_overdue:   a.is_overdue(),
    })
}
