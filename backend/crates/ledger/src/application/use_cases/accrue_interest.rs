use rust_decimal::Decimal;
use shared::domain::{AggregateRoot, UserId};

use crate::application::error::LedgerError;
use crate::application::ports::{LedgerUnitOfWork, WriteOperation};
use crate::domain::account_id::AccountId;
use crate::domain::ledger_entry::{EntryType, LedgerEntry};
use crate::domain::ledger_events::LedgerEvent;

pub struct AccrueInterestCommand {
    pub owner_id:   UserId,
    pub account_id: AccountId,
}

pub async fn execute(
    uow: &dyn LedgerUnitOfWork,
    cmd: AccrueInterestCommand,
) -> Result<(), LedgerError> {
    let mut finances = uow.load(cmd.owner_id).await?;

    let currency_str = finances
        .find_account(cmd.account_id)
        .ok_or(LedgerError::AccountNotFound(cmd.account_id.uuid()))?
        .currency();

    finances.accrue_interest(cmd.account_id)?;

    // Pull events to read the accrued amount the domain computed
    let events  = finances.pull_events();
    let accrued = events
        .iter()
        .find_map(|e| {
            if let LedgerEvent::InterestAccrued(ev) = e {
                if ev.account_id == cmd.account_id {
                    return Some(ev.amount.amount());
                }
            }
            None
        })
        .unwrap_or(Decimal::ZERO);

    let entries = if accrued > Decimal::ZERO {
        vec![LedgerEntry::new(
            cmd.account_id,
            EntryType::InterestAccrued,
            accrued.to_string(),
            format!("{:?}", currency_str),
            Some("Interest Accrued".to_string()),
            None,
        )]
    } else {
        vec![]
    };

    uow.commit(WriteOperation::new(&finances, entries)).await
}
