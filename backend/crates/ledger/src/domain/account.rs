use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::money::Money;

pub trait Account {
    fn account_id(&self) -> AccountId;
    fn account_name(&self) -> &AccountName;
    fn balance(&self) -> &Money;
    fn account_type(&self) -> &'static str;
    fn is_asset(&self) -> bool;
}
