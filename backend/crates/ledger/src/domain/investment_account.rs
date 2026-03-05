use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::account_number::AccountNumber;
use crate::domain::bank::Bank;
use crate::domain::money::Money;
use shared::domain::SharedError;
use super::account::Account;
use super::investment::Investment;
use super::ticker::Ticker;

#[derive(Debug, Clone, PartialEq)]
pub struct InvestmentAccount {
    account_id: AccountId,
    account_name: AccountName,
    account_number: AccountNumber,
    bank: Bank,

    cash_balance: Money,

    holdings: Vec<Investment>,
}

impl InvestmentAccount {
    pub fn new(
        account_id: AccountId,
        account_name: AccountName,
        account_number: AccountNumber,
        bank: Bank,
        cash_balance: Money,
    ) -> Self {
        Self {
            account_id,
            account_name,
            account_number,
            bank,
            cash_balance,
            holdings: Vec::new(),
        }
    }

    pub fn bank(&self) -> &Bank { &self.bank }
    pub fn account_number(&self) -> &AccountNumber { &self.account_number }
    pub fn cash_balance(&self) -> &Money { &self.cash_balance }
    pub fn holdings(&self) -> &[Investment] { &self.holdings }

    pub fn find_holding(&self, ticker: &Ticker) -> Option<&Investment> {
        self.holdings.iter().find(|h| h.ticker() == ticker)
    }

    pub fn add_holding(&mut self, investment: Investment) -> Result<(), SharedError> {
        if let Some(existing) = self.holdings
            .iter_mut()
            .find(|h| h.ticker() == investment.ticker())
        {
            existing.add_quantity(investment.quantity())?;
        } else {
            self.holdings.push(investment);
        }
        Ok(())
    }

    pub fn remove_holding(&mut self, ticker: &Ticker) -> Result<(), SharedError> {
        let pos = self.holdings
            .iter()
            .position(|h| h.ticker() == ticker)
            .ok_or(SharedError::Operational(
                "[InvestmentAccount] holding not found"
            ))?;
        self.holdings.remove(pos);
        Ok(())
    }

    pub fn deposit_cash(&mut self, amount: Money) -> Result<(), SharedError> {
        self.cash_balance = self.cash_balance.add(&amount)?;
        Ok(())
    }

    pub fn withdraw_cash(&mut self, amount: Money) -> Result<(), SharedError> {
        self.cash_balance = self.cash_balance.sub(&amount)?;
        Ok(())
    }

    pub fn holdings_value(&self) -> Result<Money, SharedError> {
        let mut total = Money::new(
            rust_decimal::Decimal::ZERO,
            self.cash_balance.currency,
        )?;
        for holding in &self.holdings {
            total = total.add(&holding.market_value()?)?;
        }
        Ok(total)
    }

    pub fn total_value(&self) -> Result<Money, SharedError> {
        self.cash_balance.add(&self.holdings_value()?)
    }
}

impl Account for InvestmentAccount {
    fn account_id(&self) -> AccountId { self.account_id }
    fn account_name(&self) -> &AccountName { &self.account_name }

    fn balance(&self) -> &Money { &self.cash_balance }

    fn account_type(&self) -> &'static str { "investment" }
    fn is_asset(&self) -> bool { true }
}
