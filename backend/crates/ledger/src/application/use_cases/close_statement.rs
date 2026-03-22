use rust_decimal::Decimal;
use shared::domain::AggregateRoot;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::UserFinancesRepository;
use crate::domain::account_id::AccountId;
use crate::domain::currency::Currency;
use crate::domain::liability::Liability;

pub struct CloseStatementCommand {
    pub owner_id:        UserId,
    pub account_id:      AccountId,
    pub minimum_payment: Option<Decimal>,
    pub currency:        Currency,
}

/// Returns the statement balance that was recorded at close time.
/// The domain now returns it directly — no re-reading from the aggregate needed.
pub async fn execute(
    repo: &dyn UserFinancesRepository,
    cmd: CloseStatementCommand,
) -> Result<Decimal, LedgerError> {
    let mut finances = repo.load(cmd.owner_id).await?;

    let min = cmd.minimum_payment
        .map(|a| Liability::new(a, cmd.currency))
        .transpose()?;

    // Domain returns the statement balance it just recorded
    let statement_balance = finances.close_statement(cmd.account_id, min)?;
    repo.save(&finances).await?;
    let _events = finances.pull_events(); // TODO: dispatch via EventPublisher port

    Ok(statement_balance.amount())
}
