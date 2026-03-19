use rust_decimal::Decimal;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::UserFinancesRepository;
use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::account_number::AccountNumber;
use crate::domain::bank::Bank;
use crate::domain::currency::Currency;
use crate::domain::investment_account::InvestmentAccount;
use crate::domain::money::Money;

pub struct OpenInvestmentAccountCommand {
    pub owner_id:       UserId,
    pub name:           String,
    pub account_number: String,
    pub bank:           String,
    pub currency:       Currency,
    /// Starting cash balance in the brokerage account.
    pub cash_balance:   Decimal,
}

pub async fn execute(
    repo: &dyn UserFinancesRepository,
    cmd: OpenInvestmentAccountCommand,
) -> Result<(), LedgerError> {
    let mut finances = repo.load(cmd.owner_id).await?;

    let account = InvestmentAccount::new(
        AccountId::create(),
        AccountName::new(cmd.name)?,
        AccountNumber::new(cmd.account_number)?,
        Bank::new(cmd.bank)?,
        Money::new(cmd.cash_balance, cmd.currency)?,
    );

    finances.add_investment_account(account)?;
    repo.save(&finances).await
}
