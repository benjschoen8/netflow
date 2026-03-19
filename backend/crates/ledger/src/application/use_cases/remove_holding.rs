use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::UserFinancesRepository;
use crate::domain::account_id::AccountId;
use crate::domain::ticker::Ticker;

pub struct RemoveHoldingCommand {
    pub owner_id:   UserId,
    pub account_id: AccountId,
    pub ticker:     String,
}

pub async fn execute(
    repo: &dyn UserFinancesRepository,
    cmd: RemoveHoldingCommand,
) -> Result<(), LedgerError> {
    let mut finances = repo.load(cmd.owner_id).await?;
    let ticker = Ticker::new(cmd.ticker)?;
    finances.remove_holding(cmd.account_id, &ticker)?;
    repo.save(&finances).await
}
