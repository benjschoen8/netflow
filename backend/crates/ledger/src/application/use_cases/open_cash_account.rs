use rust_decimal::Decimal;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::UserFinancesRepository;
use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::account_number::AccountNumber;
use crate::domain::bank::Bank;
use crate::domain::cash_account::CashAccount;
use crate::domain::currency::Currency;
use crate::domain::money::Money;

pub struct OpenCashAccountCommand {
    pub owner_id:      UserId,
    pub name:          String,
    pub account_number: String,
    pub bank:          String,
    pub currency:      Currency,
    pub initial_balance: Decimal,
}

pub async fn execute(
    repo: &dyn UserFinancesRepository,
    cmd: OpenCashAccountCommand,
) -> Result<(), LedgerError> {
    let mut finances = repo.load(cmd.owner_id).await?;

    let account = CashAccount::new(
        AccountId::create(),
        AccountName::new(cmd.name)?,
        AccountNumber::new(cmd.account_number)?,
        Bank::new(cmd.bank)?,
        Money::new(cmd.initial_balance, cmd.currency)?,
    );

    finances.add_cash_account(account)?;
    repo.save(&finances).await
}
