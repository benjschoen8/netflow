use rust_decimal::Decimal;
use shared::domain::AggregateRoot;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::parse_helpers::parse_wallet_provider;
use crate::application::ports::{LedgerUnitOfWork, WriteOperation};
use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::currency::Currency;
use crate::domain::digital_wallet::{DigitalWallet, ProviderAccountId};
use crate::domain::money::Money;

pub struct AddDigitalWalletCommand {
    pub owner_id:            UserId,
    pub name:                String,
    /// "line-pay", "apple-pay", "google-pay", "jko-pay", "pi-wallet", "taiwan-pay", or other
    pub provider:            String,
    pub provider_account_id: String,
    pub currency:            Currency,
    pub initial_balance:     Decimal,
}

pub async fn execute(
    uow: &dyn LedgerUnitOfWork,
    cmd: AddDigitalWalletCommand,
) -> Result<(), LedgerError> {
    let mut finances = uow.load(cmd.owner_id).await?;

    let wallet = DigitalWallet::new(
        AccountId::create(),
        AccountName::new(cmd.name)?,
        parse_wallet_provider(&cmd.provider)?,    // String → DigitalWalletProvider here
        ProviderAccountId::new(cmd.provider_account_id)?,
        Money::new(cmd.initial_balance, cmd.currency)?,
    );

    finances.add_digital_wallet(wallet)?;
    let _events = finances.pull_events();
    uow.commit(WriteOperation::new(&finances, vec![])).await
}
