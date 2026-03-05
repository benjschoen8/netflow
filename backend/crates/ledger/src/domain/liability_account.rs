use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::account_number::AccountNumber;
use crate::domain::bank::Bank;
use crate::domain::money::Money;
use crate::domain::liability::Liability;
use shared::domain::SharedError;
use super::account::Account;

#[derive(Debug, Clone, PartialEq)]
pub struct LiabilityAccount {
    account_id: AccountId,
    account_name: AccountName,
    account_number: AccountNumber,
    bank: Bank,
    liability: Liability,
}

impl LiabilityAccount {
    pub fn new(
        account_id: AccountId,
        account_name: AccountName,
        account_number: AccountNumber,
        bank: Bank,
        liability: Liability,
    ) -> Self {
        Self { account_id, account_name, account_number, bank, liability }
    }

    pub fn bank(&self) -> &Bank { &self.bank }
    pub fn account_number(&self) -> &AccountNumber { &self.account_number }
    pub fn liability(&self) -> &Liability { &self.liability }
    pub fn is_settled(&self) -> bool { self.liability.is_settled() }

    pub fn make_payment(&mut self, payment: Money) -> Result<(), SharedError> {
        self.liability = self.liability.make_payment(&payment)?;
        Ok(())
    }

    pub fn accrue_interest(&mut self) -> Result<(), SharedError> {
        self.liability = self.liability.accrue_interest()?;
        Ok(())
    }

    pub fn mark_overdue(&mut self) { self.liability.mark_overdue(); }
    pub fn mark_current(&mut self) { self.liability.mark_current(); }
}

impl Account for LiabilityAccount {
    fn account_id(&self) -> AccountId { self.account_id }
    fn account_name(&self) -> &AccountName { &self.account_name }
    fn balance(&self) -> &Money { self.liability.outstanding() }
    fn account_type(&self) -> &'static str { "liability" }
    fn is_asset(&self) -> bool { false }
}
