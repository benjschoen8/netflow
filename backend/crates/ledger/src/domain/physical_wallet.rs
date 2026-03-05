use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::money::Money;
use shared::domain::SharedError;
use super::account::Account;

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

    /// Add cash to this wallet
    pub fn add_cash(&mut self, amount: Money) -> Result<(), SharedError> {
        self.balance = self.balance.add(&amount)?;
        Ok(())
    }

    /// Remove cash from this wallet — Err if insufficient funds
    pub fn remove_cash(&mut self, amount: Money) -> Result<(), SharedError> {
        self.balance = self.balance.sub(&amount)?;
        Ok(())
    }
}

impl Account for PhysicalWallet {
    fn account_id(&self) -> AccountId { self.account_id }
    fn account_name(&self) -> &AccountName { &self.account_name }
    fn balance(&self) -> &Money { &self.balance }
    fn account_type(&self) -> &'static str { "physical_wallet" }
    fn is_asset(&self) -> bool { true }
}
