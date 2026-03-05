use shared::domain::shared_error::SharedError;
use super::account_id::AccountId;
use super::account_name::AccountName;
use super::account_number::AccountNumber;
use super::bank::Bank;
use super::money::Money;
use super::currency::Currency;
use super::account::Account;
use super::Investment::Investment;
use super::Investment::Investment;

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

    pub fn bank(&self) -> &Bank {
        &self.bank
    }

    pub fn account_number(&self) -> &AccountNumber {
        &self.account_number
    }

    pub fn cash_balance(&self) -> &Money {
        &self.cash_balance
    }

    pub fn holdings(&self) -> &[Investment] {
        &self.holdings
    }

    pub fn find_holding(&self, ticker: &Ticker) -> Option<&Investment> {
        self.holdings.iter().find(|s| s.ticker() == ticker)
    }

    pub fn add_holding(&mut self, investment: Investment) -> Result<(), SharedError> {
        if let Some(existing) = self.holdings
            .iter_mut()
            .find(|s| s.ticker() == investment.ticker())
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
            .position(|s| s.ticker() == ticker)
            .ok_or(SharedError::Operational("[InvestmentAccount] holding not found"))?;
        self.holdings.remove(pos);
        Ok(())
    }

    pub fn securities_value(&self) -> Result<Money, SharedError> {
        let currency = self.cash_balance.currency.clone();
        let mut total = Money::new(rust_decimal::Decimal::ZERO, currency.clone())?;
        for holding in &self.holdings {
            let value = holding.market_value()?;
            total = total.add(&value)?;
        }
        Ok(total)
    }

    pub fn total_value(&self) -> Result<Money, SharedError> {
        let securities = self.securities_value()?;
        self.cash_balance.add(&securities)
    }
}

impl Account for InvestmentAccount {
    fn account_id(&self) -> AccountId {
        self.account_id
    }

    fn account_name(&self) -> &AccountName {
        &self.account_name
    }

    fn balance(&self) -> &Money {
        &self.cash_balance
    }

    fn account_type(&self) -> &'static str {
        "investment"
    }

    fn is_asset(&self) -> bool {
        true
    }
}
