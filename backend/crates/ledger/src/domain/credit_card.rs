use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::money::Money;
use crate::domain::liability::Liability;
use crate::domain::monthly_day::MonthlyDay;
use super::account::Account;
use super::card_types::{CardLastFour, CardNetwork, ExpirationDate};
use shared::domain::SharedError;

#[derive(Debug, Clone, PartialEq)]
pub struct TemporaryCreditLimit {
    amount: Money,
    expires_on: chrono::NaiveDate,
}

impl TemporaryCreditLimit {
    pub fn amount(&self) -> &Money { &self.amount }
    pub fn expires_on(&self) -> chrono::NaiveDate { self.expires_on }

    pub fn is_expired(&self) -> bool {
        chrono::Utc::now().naive_utc().date() > self.expires_on
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreditCard {
    account_id: AccountId,
    account_name: AccountName,

    card_last_four: CardLastFour,

    network: CardNetwork,

    expiration_date: ExpirationDate,

    credit_limit: Money,

    temporary_credit_limit: Option<TemporaryCreditLimit>,

    cash_advance_limit: Option<Money>,

    outstanding_balance: Liability,

    statement_balance: Option<Money>,

    statement_date: MonthlyDay,

    payment_due_date: MonthlyDay,
}

impl CreditCard {
    pub fn new(
        account_id: AccountId,
        account_name: AccountName,
        card_last_four: CardLastFour,
        network: CardNetwork,
        expiration_date: ExpirationDate,
        credit_limit: Money,
        cash_advance_limit: Option<Money>,
        outstanding_balance: Liability,
        statement_date: MonthlyDay,
        payment_due_date: MonthlyDay,
    ) -> Result<Self, SharedError> {
        if outstanding_balance.outstanding().amount > credit_limit.amount {
            return Err(SharedError::InvalidFormat(
                "[CreditCard] outstanding balance cannot exceed credit limit"
            ));
        }
        if expiration_date.is_expired() {
            return Err(SharedError::InvalidFormat("[CreditCard] card is expired"));
        }
        Ok(Self {
            account_id,
            account_name,
            card_last_four,
            network,
            expiration_date,
            credit_limit,
            temporary_credit_limit: None,
            cash_advance_limit,
            outstanding_balance,
            statement_balance: None,
            statement_date,
            payment_due_date,
        })
    }


    pub fn card_last_four(&self) -> &CardLastFour { &self.card_last_four }
    pub fn network(&self) -> &CardNetwork { &self.network }
    pub fn expiration_date(&self) -> ExpirationDate { self.expiration_date }
    pub fn credit_limit(&self) -> &Money { &self.credit_limit }
    pub fn temporary_credit_limit(&self) -> Option<&TemporaryCreditLimit> { self.temporary_credit_limit.as_ref() }
    pub fn cash_advance_limit(&self) -> Option<&Money> { self.cash_advance_limit.as_ref() }
    pub fn outstanding_balance(&self) -> &Liability { &self.outstanding_balance }
    pub fn statement_balance(&self) -> Option<&Money> { self.statement_balance.as_ref() }
    pub fn statement_date(&self) -> MonthlyDay { self.statement_date }
    pub fn payment_due_date(&self) -> MonthlyDay { self.payment_due_date }
    pub fn is_overdue(&self) -> bool { self.outstanding_balance.is_overdue() }
    pub fn is_expired(&self) -> bool { self.expiration_date.is_expired() }

    pub fn effective_credit_limit(&self) -> &Money {
        match &self.temporary_credit_limit {
            Some(temp) if !temp.is_expired() => &temp.amount,
            _ => &self.credit_limit,
        }
    }

    pub fn is_over_permanent_limit(&self) -> bool {
        self.outstanding_balance.outstanding().amount > self.credit_limit.amount
    }

    pub fn available_credit(&self) -> Result<Money, SharedError> {
        self.effective_credit_limit().sub(self.outstanding_balance.outstanding())
    }

    pub fn utilisation_percent(&self) -> rust_decimal::Decimal {
        let limit = self.effective_credit_limit();
        if limit.amount.is_zero() {
            return rust_decimal::Decimal::ZERO;
        }
        (self.outstanding_balance.outstanding().amount / limit.amount)
            * rust_decimal::Decimal::ONE_HUNDRED
    }

    pub fn grant_temporary_limit(
        &mut self,
        amount: Money,
        expires_on: chrono::NaiveDate,
    ) -> Result<(), SharedError> {
        if amount.amount <= self.credit_limit.amount {
            return Err(SharedError::Operational(
                "[CreditCard] temporary limit must exceed permanent limit"
            ));
        }
        if expires_on <= chrono::Utc::now().naive_utc().date() {
            return Err(SharedError::InvalidFormat(
                "[CreditCard] temporary limit expiry must be in the future"
            ));
        }
        self.temporary_credit_limit = Some(TemporaryCreditLimit { amount, expires_on });
        Ok(())
    }

    pub fn revoke_temporary_limit(&mut self) {
        self.temporary_credit_limit = None;
    }

    pub fn charge(&mut self, amount: Money) -> Result<(), SharedError> {
        let new_outstanding = self.outstanding_balance.outstanding().add(&amount)?;
        if new_outstanding.amount > self.effective_credit_limit().amount {
            return Err(SharedError::Operational(
                "[CreditCard] charge would exceed credit limit"
            ));
        }
        self.outstanding_balance = Liability::new(
            self.outstanding_balance.principal().clone(),
            self.outstanding_balance.creditor().to_string(),
            self.outstanding_balance.interest_rate(),
            self.outstanding_balance.due_date(),
            self.outstanding_balance.maturity_date(),
            self.outstanding_balance.minimum_payment().cloned(),
        )?;
        Ok(())
    }

    pub fn make_payment(&mut self, payment: Money) -> Result<(), SharedError> {
        self.outstanding_balance = self.outstanding_balance.make_payment(&payment)?;
        Ok(())
    }

    pub fn close_statement(&mut self) {
        self.statement_balance = Some(self.outstanding_balance.outstanding().clone());
    }

    pub fn accrue_interest(&mut self) -> Result<(), SharedError> {
        self.outstanding_balance = self.outstanding_balance.accrue_interest()?;
        Ok(())
    }

    pub fn mark_overdue(&mut self) { self.outstanding_balance.mark_overdue(); }
    pub fn mark_current(&mut self) { self.outstanding_balance.mark_current(); }
}

impl Account for CreditCard {
    fn account_id(&self) -> AccountId { self.account_id }
    fn account_name(&self) -> &AccountName { &self.account_name }
    fn balance(&self) -> &Money { self.outstanding_balance.outstanding() }
    fn account_type(&self) -> &'static str { "credit_card" }
    fn is_asset(&self) -> bool { false }
}
