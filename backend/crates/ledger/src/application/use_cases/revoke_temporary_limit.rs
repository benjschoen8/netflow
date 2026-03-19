use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::UserFinancesRepository;
use crate::domain::account_id::AccountId;

pub struct RevokeTemporaryLimitCommand {
    pub owner_id:   UserId,
    pub account_id: AccountId,
}

pub async fn execute(
    repo: &dyn UserFinancesRepository,
    cmd: RevokeTemporaryLimitCommand,
) -> Result<(), LedgerError> {
    let mut finances = repo.load(cmd.owner_id).await?;
    finances.revoke_temporary_limit(cmd.account_id)?;
    repo.save(&finances).await
}
