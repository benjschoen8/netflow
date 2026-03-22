use shared::domain::AggregateRoot;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::{LedgerUnitOfWork, WriteOperation};
use crate::domain::account_id::AccountId;

pub struct RevokeTemporaryLimitCommand {
    pub owner_id:   UserId,
    pub account_id: AccountId,
}

pub async fn execute(
    uow: &dyn LedgerUnitOfWork,
    cmd: RevokeTemporaryLimitCommand,
) -> Result<(), LedgerError> {
    let mut finances = uow.load(cmd.owner_id).await?;
    finances.revoke_temporary_limit(cmd.account_id)?;
    let _events = finances.pull_events();
    uow.commit(WriteOperation::new(&finances, vec![])).await
}
