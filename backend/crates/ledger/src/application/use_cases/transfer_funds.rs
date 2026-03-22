//! Command: transfer funds between two asset accounts owned by the same user.
//!
//! Implemented as a single atomic write that:
//!   1. Withdraws `amount` from `from_account_id`
//!   2. Deposits `amount` into `to_account_id`
//!   3. Creates two LedgerEntry records (one withdrawal, one deposit) with
//!      cross-reference labels so both accounts show the counterparty.
use rust_decimal::Decimal;
use shared::domain::AggregateRoot;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::{LedgerUnitOfWork, WriteOperation};
use crate::domain::account_id::AccountId;
use crate::domain::currency::Currency;
use crate::domain::ledger_entry::{EntryType, LedgerEntry};
use crate::domain::money::Money;

pub struct TransferFundsCommand {
    pub owner_id:       UserId,
    pub from_account_id: AccountId,
    pub to_account_id:   AccountId,
    pub amount:          Decimal,
    pub currency:        Currency,
    pub label:           Option<String>,
    pub description:     Option<String>,
}

pub async fn execute(
    uow: &dyn LedgerUnitOfWork,
    cmd: TransferFundsCommand,
) -> Result<(), LedgerError> {
    if cmd.from_account_id == cmd.to_account_id {
        return Err(LedgerError::Validation(
            "Source and destination accounts must be different".to_string(),
        ));
    }

    let mut finances = uow.load(cmd.owner_id).await?;
    let amount       = Money::new(cmd.amount, cmd.currency)?;

    // Verify both accounts exist and are asset accounts before mutating
    finances.find_account(cmd.from_account_id)
        .ok_or(LedgerError::AccountNotFound(cmd.from_account_id.uuid()))?
        .asset_balance()
        .map_err(|_| LedgerError::WrongAccountType(cmd.from_account_id.uuid()))?;

    finances.find_account(cmd.to_account_id)
        .ok_or(LedgerError::AccountNotFound(cmd.to_account_id.uuid()))?
        .asset_balance()
        .map_err(|_| LedgerError::WrongAccountType(cmd.to_account_id.uuid()))?;

    // Perform both mutations
    finances.withdraw(cmd.from_account_id, &amount)?;
    finances.deposit(cmd.to_account_id, &amount)?;
    let _events = finances.pull_events();

    let cur_str = format!("{:?}", cmd.currency);

    // Build cross-reference descriptions showing the counterparty account name
    let from_name = finances
        .find_account(cmd.from_account_id)
        .map(|a| a.account_name_str().to_string())
        .unwrap_or_default();
    let to_name = finances
        .find_account(cmd.to_account_id)
        .map(|a| a.account_name_str().to_string())
        .unwrap_or_default();

    let base_label = cmd.label.as_deref().unwrap_or("Transfer");
    let base_desc  = cmd.description.clone();

    uow.commit(WriteOperation::new(
        &finances,
        vec![
            // Outgoing entry on the source account
            LedgerEntry::new(
                cmd.from_account_id,
                EntryType::Withdrawal,
                cmd.amount.to_string(),
                cur_str.clone(),
                Some(format!("{} → {}", base_label, to_name)),
                base_desc.clone(),
            ),
            // Incoming entry on the destination account
            LedgerEntry::new(
                cmd.to_account_id,
                EntryType::Deposit,
                cmd.amount.to_string(),
                cur_str,
                Some(format!("{} ← {}", base_label, from_name)),
                base_desc,
            ),
        ],
    )).await
}
