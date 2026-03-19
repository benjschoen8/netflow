use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::UserFinancesRepository;
use crate::domain::user_finances::UserFinances;

pub struct CreateUserFinancesCommand {
    pub owner_id: UserId,
}

/// Initialise a fresh `UserFinances` aggregate for a new user.
/// Errors if a record already exists for this user.
pub async fn execute(
    repo: &dyn UserFinancesRepository,
    cmd: CreateUserFinancesCommand,
) -> Result<(), LedgerError> {
    if repo.exists(cmd.owner_id).await? {
        return Err(LedgerError::Validation(
            "Finances already initialised for this user.".into(),
        ));
    }
    let aggregate = UserFinances::new(cmd.owner_id);
    repo.save(&aggregate).await
}
