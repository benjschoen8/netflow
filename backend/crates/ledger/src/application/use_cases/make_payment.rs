use rust_decimal::Decimal;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::UserFinancesRepository;
use crate::domain::account_id::AccountId;
use crate::domain::currency::Currency;
use crate::domain::liability::Liability;

pub struct MakePaymentCommand {
    /// Asset account the payment is drawn from.
    pub owner_id:        UserId,
    pub from_account_id: AccountId,
    /// The debt account (loan or credit card) being paid.
    pub debt_account_id: AccountId,
    pub amount:          Decimal,
    pub currency:        Currency,
}

pub async fn execute(
    repo: &dyn UserFinancesRepository,
    cmd: MakePaymentCommand,
) -> Result<(), LedgerError> {
    let mut finances = repo.load(cmd.owner_id).await?;
    let amount = Liability::new(cmd.amount, cmd.currency)?;
    finances.make_payment(cmd.from_account_id, cmd.debt_account_id, &amount)?;
    repo.save(&finances).await
}
