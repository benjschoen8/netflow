use shared::domain::AggregateRoot;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::{LedgerUnitOfWork, WriteOperation};
use crate::domain::account_id::AccountId;
use crate::domain::ticker::Ticker;

pub struct RemoveHoldingCommand {
    pub owner_id:   UserId,
    pub account_id: AccountId,
    pub ticker:     String,
}

pub async fn execute(
    uow: &dyn LedgerUnitOfWork,
    cmd: RemoveHoldingCommand,
) -> Result<(), LedgerError> {
    let mut finances = uow.load(cmd.owner_id).await?;
    let ticker = Ticker::new(cmd.ticker)?;
    finances.remove_holding(cmd.account_id, &ticker)?;
    let _events = finances.pull_events();
    uow.commit(WriteOperation::new(&finances, vec![])).await
}
