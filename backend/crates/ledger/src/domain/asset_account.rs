use shared::domain::SharedError;

use crate::domain::financial_entry::FinancialEntry;
use crate::domain::money::Money;

pub trait AssetAccount: FinancialEntry {
    fn balance(&self) -> &Money;
    fn deposit(&mut self, amount: &Money) -> Result<(), SharedError>;
    fn withdraw(&mut self, amount: &Money) -> Result<(), SharedError>;
}
