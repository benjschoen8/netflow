use rust_decimal::Decimal;

use shared::domain::SharedError;
use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::account_number::AccountNumber;
use crate::domain::bank::Bank;
use crate::domain::currency::Currency;
use crate::domain::liability::Liability;
use crate::domain::loan::Loan;
use crate::domain::financial_entry::FinancialEntry;
use crate::domain::debt_account::DebtAccount;

#[derive(Debug, Clone, PartialEq)]
pub struct LoanAccount {
    account_id: AccountId,
    account_name: AccountName,
    account_number: Option<AccountNumber>,
    bank: Bank,
    loan: Loan,
    minimum_payment_paid: bool,
    is_paid: bool,
}

impl LoanAccount {
    pub fn new(
        account_id: AccountId,
        account_name: AccountName,
        account_number: Option<AccountNumber>,
        bank: Bank,
        loan: Loan,
    ) -> Self {
        let is_paid = loan.is_settled();
        Self {
            account_id,
            account_name,
            account_number,
            bank,
            loan,
            minimum_payment_paid: false,
            is_paid,
        }
    }

    pub fn bank(&self) -> &Bank { &self.bank }
    pub fn account_number(&self) -> Option<&AccountNumber> { self.account_number.as_ref() }
    pub fn loan(&self) -> &Loan { &self.loan }
}

impl FinancialEntry for LoanAccount {
    fn account_id(&self) -> AccountId { self.account_id }
    fn account_name(&self) -> &AccountName { &self.account_name }
    fn account_type(&self) -> &'static str { "loan" }
    fn currency(&self) -> Currency { self.loan.outstanding().currency() }
}

impl DebtAccount for LoanAccount {
    fn outstanding(&self) -> &Liability { self.loan.outstanding() }
    fn minimum_payment(&self) -> Option<&Liability> { self.loan.minimum_payment() }
    fn minimum_payment_paid(&self) -> bool { self.minimum_payment_paid }
    fn is_paid(&self) -> bool { self.is_paid }
    fn is_overdue(&self) -> bool { self.loan.is_overdue() }
    fn interest_rate(&self) -> Option<Decimal> { self.loan.interest_rate() }

    fn make_payment(&mut self, amount: &Liability) -> Result<(), SharedError> {
        self.loan = self.loan.apply_payment(amount)?;
        self.is_paid = self.loan.is_settled();
        if let Some(min) = self.loan.minimum_payment() {
            if amount.amount() >= min.amount() {
                self.minimum_payment_paid = true;
            }
        }
        Ok(())
    }

    fn accrue_interest(&mut self) -> Result<(), SharedError> {
        self.loan = self.loan.apply_interest()?;
        Ok(())
    }

    fn mark_overdue(&mut self) { self.loan.mark_overdue(); }
    fn mark_current(&mut self) { self.loan.mark_current(); }

    fn reset_cycle(&mut self) {
        self.minimum_payment_paid = false;
    }
}
