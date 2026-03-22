use rust_decimal::Decimal;

use shared::domain::SharedError;
use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::currency::Currency;
use crate::domain::financial_entry::FinancialEntry;
use crate::domain::debt_account::DebtAccount;
use crate::domain::revolving_credit::RevolvingCredit;
use crate::domain::liability::Liability;
use crate::domain::money::Money;
use crate::domain::monthly_day::MonthlyDay;
use super::card_last_four::CardLastFour;
use super::card_network::CardNetwork;
use super::expiration_date::ExpirationDate;

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
    statement_balance: Option<Liability>,
    statement_date: MonthlyDay,
    payment_due_date: MonthlyDay,
    interest_rate: Option<Decimal>,
    minimum_payment: Option<Liability>,
    minimum_payment_paid: bool,
    is_paid: bool,
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
        interest_rate: Option<Decimal>,
    ) -> Result<Self, SharedError> {
        if outstanding_balance.amount() > credit_limit.amount() {
            return Err(SharedError::InvalidFormat(
                "[CreditCard] outstanding balance cannot exceed credit limit"
            ));
        }
        if outstanding_balance.currency() != credit_limit.currency() {
            return Err(SharedError::Operational(
                "[CreditCard] outstanding balance and credit limit currency mismatch"
            ));
        }
        if expiration_date.is_expired() {
            return Err(SharedError::InvalidFormat("[CreditCard] card is expired"));
        }
        if let Some(rate) = interest_rate {
            if rate.is_sign_negative() {
                return Err(SharedError::InvalidFormat(
                    "[CreditCard] interest rate cannot be negative"
                ));
            }
        }
        let is_paid = outstanding_balance.is_zero();
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
            interest_rate,
            minimum_payment: None,
            minimum_payment_paid: false,
            is_paid,
        })
    }


    pub fn card_last_four(&self) -> &CardLastFour { &self.card_last_four }
    pub fn network(&self) -> &CardNetwork { &self.network }

    pub fn rename(&mut self, name: AccountName) { self.account_name = name; }
    pub fn expiration_date(&self) -> ExpirationDate { self.expiration_date }
    pub fn cash_advance_limit(&self) -> Option<&Money> { self.cash_advance_limit.as_ref() }
    pub fn statement_balance(&self) -> Option<&Liability> { self.statement_balance.as_ref() }
    pub fn statement_date(&self) -> MonthlyDay { self.statement_date }
    pub fn payment_due_date(&self) -> MonthlyDay { self.payment_due_date }
    pub fn temporary_credit_limit(&self) -> Option<&TemporaryCreditLimit> { self.temporary_credit_limit.as_ref() }
    pub fn is_expired(&self) -> bool { self.expiration_date.is_expired() }
    pub fn is_over_permanent_limit(&self) -> bool {
        self.outstanding_balance.amount() > self.credit_limit.amount()
    }

    pub fn effective_credit_limit(&self) -> &Money {
        match &self.temporary_credit_limit {
            Some(temp) if !temp.is_expired() => &temp.amount,
            _ => &self.credit_limit,
        }
    }

    pub fn grant_temporary_limit(
        &mut self,
        amount: Money,
        expires_on: chrono::NaiveDate,
    ) -> Result<(), SharedError> {
        if amount.amount() <= self.credit_limit.amount() {
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

    // ── Getters added for infrastructure layer ────────────────────────────────
    pub fn credit_limit(&self) -> &Money { &self.credit_limit }

    // ── Restore helpers (infrastructure layer use only) ───────────────────────

    /// Restore the statement balance after loading from persistence.
    pub fn restore_statement_balance(&mut self, balance: Liability) {
        self.statement_balance = Some(balance);
    }

    /// Restore the minimum payment field after loading from persistence.
    pub fn restore_minimum_payment(&mut self, payment: Liability) {
        self.minimum_payment = Some(payment);
    }
}

// ── FinancialEntry ────────────────────────────────────────────────────────────

impl FinancialEntry for CreditCard {
    fn account_id(&self) -> AccountId { self.account_id }
    fn account_name(&self) -> &AccountName { &self.account_name }
    fn account_type(&self) -> &'static str { "credit_card" }
    fn currency(&self) -> Currency { self.outstanding_balance.currency() }
}

// ── DebtAccount ───────────────────────────────────────────────────────────────

impl DebtAccount for CreditCard {
    fn outstanding(&self) -> &Liability { &self.outstanding_balance }
    fn minimum_payment(&self) -> Option<&Liability> { self.minimum_payment.as_ref() }
    fn minimum_payment_paid(&self) -> bool { self.minimum_payment_paid }
    fn is_paid(&self) -> bool { self.is_paid }
    fn is_overdue(&self) -> bool {
        if let Some(stmt) = &self.statement_balance {
            !stmt.is_zero() && !self.minimum_payment_paid
        } else {
            false
        }
    }
    fn interest_rate(&self) -> Option<Decimal> { self.interest_rate }

    fn make_payment(&mut self, amount: &Liability) -> Result<(), SharedError> {
        self.outstanding_balance = self.outstanding_balance.sub(amount)?;
        self.is_paid = self.outstanding_balance.is_zero();
        if let Some(min) = &self.minimum_payment {
            if amount.amount() >= min.amount() {
                self.minimum_payment_paid = true;
            }
        }
        Ok(())
    }

    fn accrue_interest(&mut self) -> Result<(), SharedError> {
        let Some(rate) = self.interest_rate else {
            return Ok(());
        };
        let monthly_rate = rate / Decimal::from(1200);
        let interest = self.outstanding_balance.amount()
            .checked_mul(monthly_rate)
            .ok_or(SharedError::Operational("[CreditCard] overflow accruing interest"))?;
        let new_amount = self.outstanding_balance.amount()
            .checked_add(interest)
            .ok_or(SharedError::Operational("[CreditCard] overflow adding interest"))?;
        self.outstanding_balance = Liability::new(new_amount, self.outstanding_balance.currency())?;
        Ok(())
    }

    fn mark_overdue(&mut self) { /* handled via statement/minimum_payment_paid */ }
    fn mark_current(&mut self) { self.minimum_payment_paid = true; }

    fn reset_cycle(&mut self) {
        self.minimum_payment_paid = false;
    }
}

// ── RevolvingCredit ───────────────────────────────────────────────────────────

impl RevolvingCredit for CreditCard {
    fn credit_limit(&self) -> &Money { &self.credit_limit }

    fn available_credit(&self) -> Result<Money, SharedError> {
        let outstanding_as_money = Money::new(
            self.outstanding_balance.amount(),
            self.outstanding_balance.currency(),
        )?;
        self.effective_credit_limit().sub(&outstanding_as_money)
    }

    fn utilisation_percent(&self) -> Decimal {
        let limit = self.effective_credit_limit();
        if limit.amount().is_zero() {
            return Decimal::ZERO;
        }
        (self.outstanding_balance.amount() / limit.amount()) * Decimal::ONE_HUNDRED
    }

    fn charge(&mut self, amount: &Liability) -> Result<(), SharedError> {
        let new_outstanding = self.outstanding_balance.add(amount)?;
        if new_outstanding.amount() > self.effective_credit_limit().amount() {
            return Err(SharedError::Operational(
                "[CreditCard] charge would exceed credit limit"
            ));
        }
        self.outstanding_balance = new_outstanding;
        self.is_paid = self.outstanding_balance.is_zero();
        Ok(())
    }

    fn close_statement(&mut self, minimum_payment: Option<Liability>) {
        self.statement_balance = Some(self.outstanding_balance.clone());
        self.minimum_payment = minimum_payment;
        self.minimum_payment_paid = false;
    }
}
