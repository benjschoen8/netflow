use shared::domain::UserId;
use crate::application::dto::NetWorthResult;

use crate::application::error::LedgerError;
use crate::application::ports::UserFinancesRepository;
use crate::domain::currency::Currency;

pub struct GetNetWorthQuery {
    pub owner_id: UserId,
    /// Pass `None` to compute for every currency the user holds.
    pub currency: Option<Currency>,
}


pub async fn execute(
    repo: &dyn UserFinancesRepository,
    query: GetNetWorthQuery,
) -> Result<Vec<NetWorthResult>, LedgerError> {
    let finances = repo.load(query.owner_id).await?;

    let currencies: Vec<Currency> = match query.currency {
        Some(c) => vec![c],
        None    => finances.currencies_held(),
    };

    currencies
        .into_iter()
        .map(|c| {
            use crate::domain::balance::Balance;
            let assets = finances.total_assets(c)?;
            let debts  = finances.total_liabilities(c)?;
            let nw     = finances.net_worth(c)?;

            let (nw_str, is_deficit) = match nw {
                Balance::Asset(m) => (m.amount().to_string(), false),
                Balance::Debt(l)  => (format!("-{}", l.amount()), true),
            };

            Ok(NetWorthResult {
                currency:     format!("{:?}", c),
                total_assets: assets.amount().to_string(),
                total_debts:  debts.amount().to_string(),
                net_worth:    nw_str,
                is_deficit,
            })
        })
        .collect()
}
