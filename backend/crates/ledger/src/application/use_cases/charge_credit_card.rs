use rust_decimal::Decimal;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::UserFinancesRepository;
use crate::domain::account_id::AccountId;
use crate::domain::currency::Currency;
use crate::domain::liability::Liability;

pub struct ChargeCreditCardCommand {
    pub owner_id:   UserId,
    pub account_id: AccountId,
    pub amount:     Decimal,
    pub currency:   Currency,
}

pub async fn execute(
    repo: &dyn UserFinancesRepository,
    cmd: ChargeCreditCardCommand,
) -> Result<(), LedgerError> {
    let mut finances = repo.load(cmd.owner_id).await?;
    let amount = Liability::new(cmd.amount, cmd.currency)?;
    finances.charge_credit_card(cmd.account_id, &amount)?;
    repo.save(&finances).await
}
