//! Orchestrates closing a credit card statement atomically.
//! All writes — aggregate state, statement record, ledger entry — happen in
//! one DB transaction via `uow.commit(WriteOperation { ... })`.

use chrono::{Duration, Utc};
use rust_decimal::Decimal;
use shared::domain::AggregateRoot;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::{
    LedgerEntryRepository, LedgerUnitOfWork, StatementRepository, WriteOperation,
};
use crate::domain::account_id::AccountId;
use crate::domain::currency::Currency;
use crate::domain::ledger_entry::{EntryType, LedgerEntry};
use crate::domain::liability::Liability;
use crate::domain::statement::Statement;

pub struct CloseStatementWithRecordCommand {
    pub owner_id:        UserId,
    pub account_id:      AccountId,
    pub minimum_payment: Option<Decimal>,
    pub currency:        Currency,
    pub label:           Option<String>,
    pub description:     Option<String>,
}

pub async fn execute(
    uow:            &dyn LedgerUnitOfWork,
    entry_repo:     &dyn LedgerEntryRepository,
    statement_repo: &dyn StatementRepository,
    cmd: CloseStatementWithRecordCommand,
) -> Result<(), LedgerError> {
    let today = Utc::now().date_naive();

    // ── Cycle boundary (read-only) ────────────────────────────────────────
    let cycle_start = statement_repo
        .find_latest(cmd.account_id)
        .await?
        .map(|prev| prev.cycle_end.succ_opt().unwrap_or(prev.cycle_end))
        .unwrap_or_else(|| today - Duration::days(30));

    // ── Total charges this cycle (read-only) ──────────────────────────────
    let entries        = entry_repo.list_for_account(cmd.account_id).await?;
    let cycle_start_dt = cycle_start.and_hms_opt(0, 0, 0).unwrap().and_utc();
    let cycle_end_dt   = today.and_hms_opt(23, 59, 59).unwrap().and_utc();

    let total_charged: Decimal = entries
        .iter()
        .filter(|e| {
            e.entry_type == EntryType::Charge
                && e.occurred_at >= cycle_start_dt
                && e.occurred_at <= cycle_end_dt
        })
        .filter_map(|e| e.amount.parse::<Decimal>().ok())
        .sum();

    // ── Mutate aggregate ──────────────────────────────────────────────────
    let mut finances = uow.load(cmd.owner_id).await?;

    let min = cmd.minimum_payment
        .map(|a| Liability::new(a, cmd.currency))
        .transpose()?;

    // Domain returns the statement balance it recorded
    let stmt_balance_liability = finances.close_statement(cmd.account_id, min)?;
    let statement_balance      = stmt_balance_liability.amount();
    let _events                = finances.pull_events();

    // ── Build Statement record ────────────────────────────────────────────
    let statement = Statement::new(
        cmd.account_id,
        cycle_start,
        today,
        statement_balance,
        cmd.minimum_payment,
        total_charged,
    );

    let auto_desc = match cmd.minimum_payment {
        Some(min) => format!(
            "Statement balance: {} | Min payment: {} | stmt_id: {}",
            statement_balance, min, statement.id
        ),
        None => format!(
            "Statement balance: {} | stmt_id: {}",
            statement_balance, statement.id
        ),
    };

    let entry = LedgerEntry::new(
        cmd.account_id,
        EntryType::StatementClosed,
        statement_balance.to_string(),
        format!("{:?}", cmd.currency),
        cmd.label.or(Some("Statement Closed".to_string())),
        Some(cmd.description.unwrap_or(auto_desc)),
    );

    // ── Commit atomically — aggregate + statement + entry in one tx ───────
    uow.commit(WriteOperation {
        aggregate:        &finances,
        entries:          vec![entry],
        new_statement:    Some(statement),
        credit_statement: None,
    }).await
}
