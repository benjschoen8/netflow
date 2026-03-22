use shared::domain::SharedError;
use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::account_number::AccountNumber;
use crate::domain::bank::Bank;
use crate::domain::currency::Currency;
use crate::domain::money::Money;
use crate::domain::financial_entry::FinancialEntry;
use crate::domain::asset_account::AssetAccount;

#[derive(Debug, Clone, PartialEq)]
pub struct CashAccount {
    account_id: AccountId,
    account_name: AccountName,
    account_number: AccountNumber,
    bank: Bank,
    balance: Money,
}

impl CashAccount {
    pub fn new(
        account_id: AccountId,
        account_name: AccountName,
        account_number: AccountNumber,
        bank: Bank,
        balance: Money,
    ) -> Self {
        Self { account_id, account_name, account_number, bank, balance }
    }

    pub fn account_number(&self) -> &AccountNumber { &self.account_number }
    pub fn bank(&self) -> &Bank { &self.bank }

    pub fn rename(&mut self, name: AccountName) { self.account_name = name; }
    pub fn set_bank(&mut self, bank: Bank) { self.bank = bank; }
    pub fn set_account_number(&mut self, number: AccountNumber) { self.account_number = number; }
}

impl FinancialEntry for CashAccount {
    fn account_id(&self) -> AccountId { self.account_id }
    fn account_name(&self) -> &AccountName { &self.account_name }
    fn account_type(&self) -> &'static str { "cash" }
    fn currency(&self) -> Currency { self.balance.currency() }
}

impl AssetAccount for CashAccount {
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
