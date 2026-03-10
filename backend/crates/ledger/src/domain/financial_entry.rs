use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::currency::Currency;

pub trait FinancialEntry {
    fn account_id(&self) -> AccountId;
    fn account_name(&self) -> &AccountName;
    fn account_type(&self) -> &'static str;
    fn currency(&self) -> Currency;
}
