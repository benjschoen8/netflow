use rust_decimal::Decimal;
use shared::domain::AggregateRoot;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::{
    LedgerUnitOfWork, StatementRepository, WriteOperation,
};
use crate::domain::account_id::AccountId;
use crate::domain::currency::Currency;
use crate::domain::ledger_entry::{EntryType, LedgerEntry};
use crate::domain::liability::Liability;

pub struct MakePaymentCommand {
    pub owner_id:        UserId,
    pub from_account_id: AccountId,
    pub debt_account_id: AccountId,
    pub amount:          Decimal,
    pub currency:        Currency,
    pub label:           Option<String>,
    pub description:     Option<String>,
}

pub async fn execute(
    uow:            &dyn LedgerUnitOfWork,
    statement_repo: &dyn StatementRepository,
    cmd: MakePaymentCommand,
) -> Result<(), LedgerError> {
    let mut finances = uow.load(cmd.owner_id).await?;

    let is_credit_card = finances
        .find_account(cmd.debt_account_id)
        .map(|a| a.as_credit_card().is_some())
        .unwrap_or(false);

    let amount = Liability::new(cmd.amount, cmd.currency)?;
    finances.make_payment(cmd.from_account_id, cmd.debt_account_id, &amount)?;
    let _events = finances.pull_events();

    let currency_str = format!("{:?}", cmd.currency);
    let amount_str   = cmd.amount.to_string();

    // Find the open statement BEFORE committing so we include its update
    // atomically in the same transaction.
    let credit_statement = if is_credit_card {
        statement_repo
            .find_latest_unsettled(cmd.debt_account_id)
            .await?
            .map(|stmt| (stmt.id, cmd.amount))
    } else {
        None
    };

    uow.commit(WriteOperation {
        aggregate: &finances,
        entries: vec![
            LedgerEntry::new(
                cmd.from_account_id,
                EntryType::PaymentMade,
                amount_str.clone(),
                currency_str.clone(),
                cmd.label.clone(),
                cmd.description.clone(),
            ),
            LedgerEntry::new(
                cmd.debt_account_id,
                EntryType::PaymentReceived,
                amount_str,
                currency_str,
                cmd.label,
                cmd.description,
            ),
        ],
        new_statement:    None,
        credit_statement,
    }).await
}
