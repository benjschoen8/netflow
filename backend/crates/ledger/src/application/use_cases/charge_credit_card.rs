use rust_decimal::Decimal;
use shared::domain::AggregateRoot;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::{LedgerUnitOfWork, WriteOperation};
use crate::domain::account_id::AccountId;
use crate::domain::currency::Currency;
use crate::domain::ledger_entry::{EntryType, LedgerEntry};
use crate::domain::liability::Liability;

pub struct ChargeCreditCardCommand {
    pub owner_id:    UserId,
    pub account_id:  AccountId,
    pub amount:      Decimal,
    pub currency:    Currency,
    pub label:       Option<String>,
    pub description: Option<String>,
}

pub async fn execute(
    uow: &dyn LedgerUnitOfWork,
    cmd: ChargeCreditCardCommand,
) -> Result<(), LedgerError> {
    let mut finances = uow.load(cmd.owner_id).await?;
    let amount       = Liability::new(cmd.amount, cmd.currency)?;

    finances.charge_credit_card(cmd.account_id, &amount)?;
    let _events = finances.pull_events();

    uow.commit(WriteOperation::new(
        &finances,
        vec![LedgerEntry::new(
            cmd.account_id,
            EntryType::Charge,
            cmd.amount.to_string(),
            format!("{:?}", cmd.currency),
            cmd.label,
            cmd.description,
        )],
    )).await
}
