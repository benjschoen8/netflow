use crate::domain::money::Money;

/// Extended trait for accounts that represent debt
/// All liabilities are Accounts — but not all Accounts are liabilities
pub trait Liability: super::account::Account {
    /// Total amount owed
    fn amount_owed(&self) -> &Money;

    /// Minimum payment due this period — None if not applicable
    fn minimum_payment(&self) -> Option<&Money>;

    /// Interest rate as a percentage e.g. 19.99 for 19.99%
    fn interest_rate(&self) -> Option<rust_decimal::Decimal>;

    /// Whether this liability is overdue
    fn is_overdue(&self) -> bool;
}
