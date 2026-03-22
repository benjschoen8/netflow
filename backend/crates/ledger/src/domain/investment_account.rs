use rust_decimal::Decimal;

use shared::domain::SharedError;
use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::account_number::AccountNumber;
use crate::domain::bank::Bank;
use crate::domain::currency::Currency;
use crate::domain::money::Money;
use crate::domain::financial_entry::FinancialEntry;
use crate::domain::asset_account::AssetAccount;
use crate::domain::investment::Investment;
use crate::domain::ticker::Ticker;

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
        Self { account_id, account_name, account_number, bank, cash_balance, holdings: Vec::new() }
    }

    pub fn bank(&self) -> &Bank { &self.bank }
    pub fn account_number(&self) -> &AccountNumber { &self.account_number }

    pub fn rename(&mut self, name: AccountName) { self.account_name = name; }
    pub fn set_bank(&mut self, bank: Bank) { self.bank = bank; }
    pub fn set_account_number(&mut self, number: AccountNumber) { self.account_number = number; }
    pub fn cash_balance(&self) -> &Money { &self.cash_balance }
    pub fn holdings(&self) -> &[Investment] { &self.holdings }

    pub fn find_holding(&self, ticker: &Ticker) -> Option<&Investment> {
        self.holdings.iter().find(|h| h.ticker() == ticker)
    }

    pub fn add_holding(&mut self, investment: Investment) -> Result<(), SharedError> {
        if let Some(existing) = self.holdings.iter_mut().find(|h| h.ticker() == investment.ticker()) {
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
            .ok_or(SharedError::Operational("[InvestmentAccount] holding not found"))?;
        self.holdings.remove(pos);
        Ok(())
    }

    /// Update the unit price of an existing holding.
    pub fn update_holding_price(
        &mut self,
        ticker: &Ticker,
        new_price: Money,
    ) -> Result<(), SharedError> {
        let holding = self.holdings
            .iter_mut()
            .find(|h| h.ticker() == ticker)
            .ok_or(SharedError::Operational(
                "[InvestmentAccount] holding not found — cannot update price",
            ))?;
        holding.update_price(new_price);
        Ok(())
    }

    pub fn holdings_value(&self) -> Result<Money, SharedError> {
        self.holdings.iter().try_fold(
            Money::zero(self.cash_balance.currency()),
            |acc, h| acc.add(&h.market_value()?),
        )
    }

    pub fn total_value(&self) -> Result<Money, SharedError> {
        self.cash_balance.add(&self.holdings_value()?)
    }
}

impl FinancialEntry for InvestmentAccount {
    fn account_id(&self) -> AccountId { self.account_id }
    fn account_name(&self) -> &AccountName { &self.account_name }
    fn account_type(&self) -> &'static str { "investment" }
    fn currency(&self) -> Currency { self.cash_balance.currency() }
}

/// `AssetAccount::balance` returns cash balance only.
/// Use `total_value()` for cash + holdings combined.
impl AssetAccount for InvestmentAccount {
    fn balance(&self) -> &Money { &self.cash_balance }

    fn deposit(&mut self, amount: &Money) -> Result<(), SharedError> {
        self.cash_balance = self.cash_balance.add(amount)?;
        Ok(())
    }

    fn withdraw(&mut self, amount: &Money) -> Result<(), SharedError> {
        self.cash_balance = self.cash_balance.sub(amount)?;
        Ok(())
    }
}
