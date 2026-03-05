use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::money::Money;

/// Port — every account type implements this
pub trait Account {
    fn account_id(&self) -> AccountId;
    fn account_name(&self) -> &AccountName;

    /// Current balance — positive means you have money
    fn balance(&self) -> &Money;

    /// Account type label for display and serialization
    fn account_type(&self) -> &'static str;

    /// Whether this account is included in net worth calculations
    fn is_asset(&self) -> bool;
}
