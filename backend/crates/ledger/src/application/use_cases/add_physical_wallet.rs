use rust_decimal::Decimal;
use shared::domain::AggregateRoot;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::{LedgerUnitOfWork, WriteOperation};
use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::currency::Currency;
use crate::domain::money::Money;
use crate::domain::physical_wallet::PhysicalWallet;

pub struct AddPhysicalWalletCommand {
    pub owner_id:        UserId,
    pub name:            String,
    pub currency:        Currency,
    pub initial_balance: Decimal,
}

pub async fn execute(
    uow: &dyn LedgerUnitOfWork,
    cmd: AddPhysicalWalletCommand,
) -> Result<(), LedgerError> {
    let mut finances = uow.load(cmd.owner_id).await?;

    let wallet = PhysicalWallet::new(
        AccountId::create(),
        AccountName::new(cmd.name)?,
        Money::new(cmd.initial_balance, cmd.currency)?,
    );

    finances.add_physical_wallet(wallet)?;
    let _events = finances.pull_events();
    uow.commit(WriteOperation::new(&finances, vec![])).await
}
