use rust_decimal::Decimal;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::UserFinancesRepository;
use crate::domain::account_id::AccountId;
use crate::domain::currency::Currency;
use crate::domain::money::Money;
use crate::domain::ticker::Ticker;

pub struct UpdateHoldingPriceCommand {
    pub owner_id:   UserId,
    pub account_id: AccountId,
    pub ticker:     String,
    pub new_price:  Decimal,
    pub currency:   Currency,
}

pub async fn execute(
    repo: &dyn UserFinancesRepository,
    cmd: UpdateHoldingPriceCommand,
) -> Result<(), LedgerError> {
    let mut finances = repo.load(cmd.owner_id).await?;
    let ticker    = Ticker::new(cmd.ticker)?;
    let new_price = Money::new(cmd.new_price, cmd.currency)?;
    finances.update_holding_price(cmd.account_id, &ticker, new_price)?;
    repo.save(&finances).await
}
