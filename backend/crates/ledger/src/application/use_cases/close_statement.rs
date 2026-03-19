use rust_decimal::Decimal;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::UserFinancesRepository;
use crate::domain::account_id::AccountId;
use crate::domain::currency::Currency;
use crate::domain::liability::Liability;

pub struct CloseStatementCommand {
    pub owner_id:        UserId,
    pub account_id:      AccountId,
    /// If set, the bank-calculated minimum payment due this cycle.
    pub minimum_payment: Option<Decimal>,
    pub currency:        Currency,
}

pub async fn execute(
    repo: &dyn UserFinancesRepository,
    cmd: CloseStatementCommand,
) -> Result<(), LedgerError> {
    let mut finances = repo.load(cmd.owner_id).await?;

    let min = cmd.minimum_payment
        .map(|a| Liability::new(a, cmd.currency))
        .transpose()?;

    finances.close_statement(cmd.account_id, min)?;
    repo.save(&finances).await
}
