use rust_decimal::Decimal;
use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::account_number::AccountNumber;
use crate::domain::bank::Bank;
use crate::domain::money::Money;
use crate::domain::shared_error::SharedError;
use super::account::Account;
use super::liability::Liability;

/// A general liability — e.g. a personal loan or mortgage
#[derive(Debug, Clone, PartialEq)]
pub struct LiabilityAccount {
    account_id: AccountId,
    account_name: AccountName,
    account_number: AccountNumber,
    bank: Bank,

    /// Amount still owed — always positive
    amount_owed: Money,

    /// Monthly minimum payment
    minimum_payment: Option<Money>,

    /// Annual interest rate e.g. Decimal::from_str("5.5") for 5.5%
    interest_rate: Option<Decimal>,

    overdue: bool,
}

impl LiabilityAccount {
    pub fn new(
        account_id: AccountId,
        account_name: AccountName,
        account_number: AccountNumber,
        bank: Bank,
        amount_owed: Money,
        minimum_payment: Option<Money>,
        interest_rate: Option<Decimal>,
    ) -> Self {
        Self {
            account_id,
            account_name,
            account_number,
            bank,
            amount_owed,
            minimum_payment,
            interest_rate,
            overdue: false,
        }
    }

    pub fn bank(&self) -> &Bank {
        &self.bank
    }

    pub fn account_number(&self) -> &AccountNumber {
        &self.account_number
    }

    pub fn mark_overdue(&mut self) {
        self.overdue = true;
    }

    pub fn mark_current(&mut self) {
        self.overdue = false;
    }

    /// Record a payment — reduces amount owed
    pub fn make_payment(&mut self, payment: Money) -> Result<(), SharedError> {
        self.amount_owed = self.amount_owed.sub(&payment)?;
        Ok(())
    }
}

impl Account for LiabilityAccount {
    fn account_id(&self) -> AccountId {
        self.account_id
    }

    fn account_name(&self) -> &AccountName {
        &self.account_name
    }

    /// For liabilities, balance is negative — represents what you owe
    fn balance(&self) -> &Money {
        &self.amount_owed
    }

    fn account_type(&self) -> &'static str {
        "liability"
    }

    fn is_asset(&self) -> bool {
        false
    }
}

impl Liability for LiabilityAccount {
    fn amount_owed(&self) -> &Money {
        &self.amount_owed
    }

    fn minimum_payment(&self) -> Option<&Money> {
        self.minimum_payment.as_ref()
    }

    fn interest_rate(&self) -> Option<Decimal> {
        self.interest_rate
    }

    fn is_overdue(&self) -> bool {
        self.overdue
    }
}
