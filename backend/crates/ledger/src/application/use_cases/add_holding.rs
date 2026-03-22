use rust_decimal::Decimal;
use shared::domain::AggregateRoot;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::parse_helpers::parse_investment_type;
use crate::application::ports::{LedgerUnitOfWork, WriteOperation};
use crate::domain::account_id::AccountId;
use crate::domain::currency::Currency;
use crate::domain::investment::Investment;
use crate::domain::money::Money;
use crate::domain::ticker::Ticker;

pub struct AddHoldingCommand {
    pub owner_id:        UserId,
    pub account_id:      AccountId,
    pub ticker:          String,
    /// "stock", "etf", "mutual-fund", "bond", "crypto", or other
    pub investment_type: String,
    pub quantity:        Decimal,
    pub unit_price:      Decimal,
    pub currency:        Currency,
}

pub async fn execute(
    uow: &dyn LedgerUnitOfWork,
    cmd: AddHoldingCommand,
) -> Result<(), LedgerError> {
    let mut finances = uow.load(cmd.owner_id).await?;

    let investment = Investment::new(
        Ticker::new(cmd.ticker)?,
        parse_investment_type(&cmd.investment_type)?,   // String → InvestmentType here
        cmd.quantity,
        Money::new(cmd.unit_price, cmd.currency)?,
    )?;

    finances.add_holding(cmd.account_id, investment)?;
    let _events = finances.pull_events();
    uow.commit(WriteOperation::new(&finances, vec![])).await
}
