//! Command: patch the display-info fields of any account.
//!
//! All fields are optional — only `Some(...)` values are applied.
//! This is deliberate patch semantics: a single endpoint handles any
//! combination of name / bank / account-number changes without the caller
//! having to send the full current state.

use shared::domain::AggregateRoot;
use shared::domain::UserId;
use uuid::Uuid;

use crate::application::error::LedgerError;
use crate::application::ports::{LedgerUnitOfWork, WriteOperation};
use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::account_number::AccountNumber;
use crate::domain::bank::Bank;

pub struct UpdateAccountInfoCommand {
    pub owner_id:       UserId,
    pub account_id:     Uuid,
    /// New display name, or `None` to leave unchanged.
    pub name:           Option<String>,
    /// New bank label (cash / investment / loan accounts only), or `None`.
    pub bank:           Option<String>,
    /// New account number, or `None`.
    pub account_number: Option<String>,
}

pub async fn execute(
    uow: &dyn LedgerUnitOfWork,
    cmd: UpdateAccountInfoCommand,
) -> Result<(), LedgerError> {
    if cmd.name.is_none() && cmd.bank.is_none() && cmd.account_number.is_none() {
        return Err(LedgerError::Validation(
            "At least one field (name, bank, account_number) must be provided".to_string(),
        ));
    }

    let mut finances = uow.load(cmd.owner_id).await?;
    let account_id   = AccountId::restore(cmd.account_id)
        .map_err(LedgerError::Domain)?;

    let name = cmd.name
        .map(AccountName::new)
        .transpose()
        .map_err(LedgerError::Domain)?;

    let bank = cmd.bank
        .map(Bank::new)
        .transpose()
        .map_err(LedgerError::Domain)?;

    let account_number = cmd.account_number
        .map(AccountNumber::new)
        .transpose()
        .map_err(LedgerError::Domain)?;

    finances
        .update_account_info(account_id, name, bank, account_number)
        .map_err(LedgerError::Domain)?;

    let _events = finances.pull_events();
    uow.commit(WriteOperation::new(&finances, vec![])).await
}
