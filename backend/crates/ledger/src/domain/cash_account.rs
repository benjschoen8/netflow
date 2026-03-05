use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::account_number::AccountNumber;
use crate::domain::bank::Bank;
use crate::domain::money::Money;
use crate::domain::shared_error::SharedError;
use super::account::Account;

/// A standard bank account — chequing or savings
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
        Self {
            account_id,
            account_name,
            account_number,
            bank,
            balance,
        }
    }

    pub fn account_number(&self) -> &AccountNumber {
        &self.account_number
    }

    pub fn bank(&self) -> &Bank {
        &self.bank
    }

    /// Deposit funds — returns new balance
    pub fn deposit(&mut self, amount: Money) -> Result<&Money, SharedError> {
        self.balance = self.balance.add(&amount)?;
        Ok(&self.balance)
    }

    /// Withdraw funds — returns new balance
    pub fn withdraw(&mut self, amount: Money) -> Result<&Money, SharedError> {
        self.balance = self.balance.sub(&amount)?;
        Ok(&self.balance)
    }
}

impl Account for CashAccount {
    fn account_id(&self) -> AccountId {
        self.account_id
    }

    fn account_name(&self) -> &AccountName {
        &self.account_name
    }

    fn balance(&self) -> &Money {
        &self.balance
    }

    fn account_type(&self) -> &'static str {
        "cash"
    }

    fn is_asset(&self) -> bool {
        true
    }
}
