use rust_decimal::Decimal;
use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::account_number::AccountNumber;
use crate::domain::bank::Bank;
use crate::domain::money::Money;
use crate::domain::shared_error::SharedError;
use super::account::Account;
use super::liability::Liability;

/// A credit card account
/// Balance = amount currently owed (outstanding charges)
/// Credit limit - balance = available credit
#[derive(Debug, Clone, PartialEq)]
pub struct CreditCard {
    account_id: AccountId,
    account_name: AccountName,
    account_number: AccountNumber,
    bank: Bank,

    /// Current outstanding balance — what you owe right now
    balance: Money,

    /// Maximum credit limit
    credit_limit: Money,

    /// Minimum payment due this statement period
    minimum_payment: Option<Money>,

    /// Annual interest rate e.g. 19.99
    interest_rate: Option<Decimal>,

    overdue: bool,
}

impl CreditCard {
    pub fn new(
        account_id: AccountId,
        account_name: AccountName,
        account_number: AccountNumber,
        bank: Bank,
        balance: Money,
        credit_limit: Money,
        minimum_payment: Option<Money>,
        interest_rate: Option<Decimal>,
    ) -> Result<Self, SharedError> {
        // Balance cannot exceed credit limit
        if balance.amount > credit_limit.amount {
            return Err(SharedError::InvalidFormat(
                "[CreditCard] balance cannot exceed credit limit"
            ));
        }
        Ok(Self {
            account_id,
            account_name,
            account_number,
            bank,
            balance,
            credit_limit,
            minimum_payment,
            interest_rate,
            overdue: false,
        })
    }

    pub fn bank(&self) -> &Bank {
        &self.bank
    }

    pub fn account_number(&self) -> &AccountNumber {
        &self.account_number
    }

    pub fn credit_limit(&self) -> &Money {
        &self.credit_limit
    }

    /// How much credit is still available
    pub fn available_credit(&self) -> Result<Money, SharedError> {
        self.credit_limit.sub(&self.balance)
    }

    /// Utilisation as a percentage 0.0 - 100.0
    pub fn utilisation_percent(&self) -> Decimal {
        if self.credit_limit.amount.is_zero() {
            return Decimal::ZERO;
        }
        (self.balance.amount / self.credit_limit.amount) * Decimal::ONE_HUNDRED
    }

    /// Charge to the card — increases balance
    pub fn charge(&mut self, amount: Money) -> Result<(), SharedError> {
        let new_balance = self.balance.add(&amount)?;
        if new_balance.amount > self.credit_limit.amount {
            return Err(SharedError::Operational(
                "[CreditCard] charge would exceed credit limit"
            ));
        }
        self.balance = new_balance;
        Ok(())
    }

    /// Make a payment — reduces balance
    pub fn make_payment(&mut self, amount: Money) -> Result<(), SharedError> {
        self.balance = self.balance.sub(&amount)?;
        Ok(())
    }

    pub fn mark_overdue(&mut self) {
        self.overdue = true;
    }

    pub fn mark_current(&mut self) {
        self.overdue = false;
    }
}

impl Account for CreditCard {
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
        "credit_card"
    }

    fn is_asset(&self) -> bool {
        false
    }
}

impl Liability for CreditCard {
    fn amount_owed(&self) -> &Money {
        &self.balance
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
