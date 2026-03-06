use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::account_number::AccountNumber;
use crate::domain::bank::Bank;
use crate::domain::money::Money;
use crate::domain::liability::Liability;
use crate::domain::loan::Loan;
use shared::domain::SharedError;
use super::account::Account;

#[derive(Debug, Clone, PartialEq)]
pub struct LoanAccount {
    account_id: AccountId,
    account_name: AccountName,
    account_number: AccountNumber,
    bank: Bank,
    loan: Loan,
}

impl LoanAccount {
    pub fn new(
        account_id: AccountId,
        account_name: AccountName,
        account_number: AccountNumber,
        bank: Bank,
        loan: Loan,
    ) -> Self {
        Self { account_id, account_name, account_number, bank, loan }
    }

    pub fn bank(&self) -> &Bank { &self.bank }
    pub fn account_number(&self) -> &AccountNumber { &self.account_number }
    pub fn loan(&self) -> &Loan { &self.loan }
    pub fn is_settled(&self) -> bool { self.loan.is_settled() }

    pub fn make_payment(&mut self, payment: Money) -> Result<(), SharedError> {
        self.loan = self.loan.make_payment(&payment)?;
        Ok(())
    }

    pub fn accrue_interest(&mut self) -> Result<(), SharedError> {
        self.loan = self.loan.accrue_interest()?;
        Ok(())
    }

    pub fn mark_overdue(&mut self) { self.loan.mark_overdue(); }
    pub fn mark_current(&mut self) { self.loan.mark_current(); }
}

impl Account for LoanAccount {
    fn account_id(&self) -> AccountId { self.account_id }
    fn account_name(&self) -> &AccountName { &self.account_name }
    fn balance(&self) -> &Liability { self.loan.outstanding() }
    fn account_type(&self) -> &'static str { "liability" }
    fn is_asset(&self) -> bool { false }
}
