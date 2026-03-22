use rust_decimal::Decimal;
use chrono::NaiveDate;
use shared::domain::AggregateRoot;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::{LedgerUnitOfWork, WriteOperation};
use crate::domain::account_id::AccountId;
use crate::domain::currency::Currency;
use crate::domain::money::Money;

pub struct GrantTemporaryLimitCommand {
    pub owner_id:   UserId,
    pub account_id: AccountId,
    pub new_limit:  Decimal,
    pub currency:   Currency,
    pub expires_on: NaiveDate,
}

pub async fn execute(
    uow: &dyn LedgerUnitOfWork,
    cmd: GrantTemporaryLimitCommand,
) -> Result<(), LedgerError> {
    let mut finances = uow.load(cmd.owner_id).await?;
    let new_limit = Money::new(cmd.new_limit, cmd.currency)?;
    finances.grant_temporary_limit(cmd.account_id, new_limit, cmd.expires_on)?;
    let _events = finances.pull_events();
    uow.commit(WriteOperation::new(&finances, vec![])).await
}
