use shared::domain::AggregateRoot;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::{LedgerUnitOfWork, WriteOperation};
use crate::domain::user_finances::UserFinances;

pub struct CreateUserFinancesCommand {
    pub owner_id: UserId,
}

/// Initialise a fresh `UserFinances` aggregate for a new user.
/// Errors if a record already exists for this user.
pub async fn execute(
    uow: &dyn LedgerUnitOfWork,
    cmd: CreateUserFinancesCommand,
) -> Result<(), LedgerError> {
    if uow.exists(cmd.owner_id).await? {
        return Err(LedgerError::Validation(
            "Finances already initialised for this user.".into(),
        ));
    }
    let mut aggregate = UserFinances::new(cmd.owner_id);
    let _events = aggregate.pull_events();
    uow.commit(WriteOperation::new(&aggregate, vec![])).await
}
