use shared::domain::SharedError;
use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::currency::Currency;
use crate::domain::money::Money;
use crate::domain::financial_entry::FinancialEntry;
use crate::domain::asset_account::AssetAccount;

#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalWallet {
    account_id: AccountId,
    account_name: AccountName,
    balance: Money,
}

impl PhysicalWallet {
    pub fn new(
        account_id: AccountId,
        account_name: AccountName,
        balance: Money,
    ) -> Self {
        Self { account_id, account_name, balance }
    }

    pub fn rename(&mut self, name: AccountName) { self.account_name = name; }
}

impl FinancialEntry for PhysicalWallet {
    fn account_id(&self) -> AccountId { self.account_id }
    fn account_name(&self) -> &AccountName { &self.account_name }
    fn account_type(&self) -> &'static str { "physical_wallet" }
    fn currency(&self) -> Currency { self.balance.currency() }
}

impl AssetAccount for PhysicalWallet {
    fn balance(&self) -> &Money { &self.balance }

    fn deposit(&mut self, amount: &Money) -> Result<(), SharedError> {
        self.balance = self.balance.add(amount)?;
        Ok(())
    }

    fn withdraw(&mut self, amount: &Money) -> Result<(), SharedError> {
        self.balance = self.balance.sub(amount)?;
        Ok(())
    }
}
