use rust_decimal::Decimal;
use shared::domain::SharedError;

use crate::domain::financial_entry::FinancialEntry;
use crate::domain::liability::Liability;

/// Any account that tracks a debt obligation.
/// Implemented by: LoanAccount, CreditCard.
/// `make_payment` only mutates this account — the use case is responsible
/// for debiting the source asset account atomically within a DB transaction.
pub trait DebtAccount: FinancialEntry {
    fn outstanding(&self) -> &Liability;
    fn minimum_payment(&self) -> Option<&Liability>;
    fn minimum_payment_paid(&self) -> bool;
    fn is_paid(&self) -> bool;
    fn is_overdue(&self) -> bool;
    fn interest_rate(&self) -> Option<Decimal>;

    fn make_payment(&mut self, amount: &Liability) -> Result<(), SharedError>;
    fn accrue_interest(&mut self) -> Result<(), SharedError>;
    fn mark_overdue(&mut self);
    fn mark_current(&mut self);
    fn reset_cycle(&mut self);
}
