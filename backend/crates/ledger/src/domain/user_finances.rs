use rust_decimal::Decimal;

use shared::domain::UserId;
use crate::domain::account_id::AccountId;
use crate::domain::currency::Currency;
use crate::domain::money::Money;
use crate::domain::signed_money::SignedMoney;
use shared::domain::SharedError;
use super::financial_account::FinancialAccount;
use super::cash_account::CashAccount;
use super::investment_account::InvestmentAccount;
use super::credit_card::CreditCard;
use super::liability_account::LiabilityAccount;
use super::physical_wallet::PhysicalWallet;
use super::digital_wallet::DigitalWallet;

#[derive(Debug, Clone, PartialEq)]
pub struct UserFinances {
    owner_id: UserId,
    accounts: Vec<FinancialAccount>,
}

impl UserFinances {
    pub fn new(owner_id: UserId) -> Self {
        Self { owner_id, accounts: Vec::new() }
    }

    // ── Getters ───────────────────────────────────────────────────────────────

    pub fn owner_id(&self) -> UserId { self.owner_id }
    pub fn accounts(&self) -> &[FinancialAccount] { &self.accounts }

    // ── Account management ────────────────────────────────────────────────────

    /// Add any account type — accepts concrete types directly via From impls:
    /// finances.add_account(my_physical_wallet)?
    /// finances.add_account(my_credit_card)?
    pub fn add_account(
        &mut self,
        account: impl Into<FinancialAccount>,
    ) -> Result<(), SharedError> {
        let account = account.into();
        if self.accounts.iter().any(|a| a.account_id() == account.account_id()) {
            return Err(SharedError::Operational(
                "[UserFinances] account with this id already exists"
            ));
        }
        self.accounts.push(account);
        Ok(())
    }

    pub fn remove_account(&mut self, id: AccountId) -> Result<(), SharedError> {
        let pos = self.accounts
            .iter()
            .position(|a| a.account_id() == id)
            .ok_or(SharedError::Operational(
                "[UserFinances] account not found"
            ))?;
        self.accounts.remove(pos);
        Ok(())
    }

    pub fn find_account(&self, id: AccountId) -> Option<&FinancialAccount> {
        self.accounts.iter().find(|a| a.account_id() == id)
    }

    pub fn find_account_mut(&mut self, id: AccountId) -> Option<&mut FinancialAccount> {
        self.accounts.iter_mut().find(|a| a.account_id() == id)
    }

    // ── Filtered views ────────────────────────────────────────────────────────

    pub fn physical_wallets(&self) -> impl Iterator<Item = &PhysicalWallet> {
        self.accounts.iter().filter_map(|a| a.as_physical_wallet())
    }

    pub fn digital_wallets(&self) -> impl Iterator<Item = &DigitalWallet> {
        self.accounts.iter().filter_map(|a| a.as_digital_wallet())
    }

    pub fn cash_accounts(&self) -> impl Iterator<Item = &CashAccount> {
        self.accounts.iter().filter_map(|a| a.as_cash())
    }

    pub fn investment_accounts(&self) -> impl Iterator<Item = &InvestmentAccount> {
        self.accounts.iter().filter_map(|a| a.as_investment())
    }

    pub fn credit_cards(&self) -> impl Iterator<Item = &CreditCard> {
        self.accounts.iter().filter_map(|a| a.as_credit_card())
    }

    pub fn liability_accounts(&self) -> impl Iterator<Item = &LiabilityAccount> {
        self.accounts.iter().filter_map(|a| a.as_liability())
    }

    pub fn overdue_accounts(&self) -> impl Iterator<Item = &FinancialAccount> {
        self.accounts.iter().filter(|a| a.is_overdue())
    }

    // ── Net worth calculations ────────────────────────────────────────────────
    // Per-currency — pass the currency you want to calculate in.
    // Use currencies_held() to know which currencies exist.

    /// Total value of all assets in a given currency
    pub fn total_assets(&self, currency: Currency) -> Result<Money, SharedError> {
        self.accounts
            .iter()
            .filter(|a| a.is_asset() && a.balance().currency == currency)
            .try_fold(
                Money::new(Decimal::ZERO, currency)?,
                |acc, a| acc.add(a.balance()),
            )
    }

    /// Total value of all liabilities in a given currency
    pub fn total_liabilities(&self, currency: Currency) -> Result<Money, SharedError> {
        self.accounts
            .iter()
            .filter(|a| !a.is_asset() && a.balance().currency == currency)
            .try_fold(
                Money::new(Decimal::ZERO, currency)?,
                |acc, a| acc.add(a.balance()),
            )
    }

    /// Net worth = assets - liabilities for a given currency.
    /// Returns SignedMoney because net worth can be negative.
    pub fn net_worth(&self, currency: Currency) -> Result<SignedMoney, SharedError> {
        let assets = self.total_assets(currency)?;
        let liabilities = self.total_liabilities(currency)?;
        let amount = assets.amount
            .checked_sub(liabilities.amount)
            .ok_or(SharedError::Operational(
                "[UserFinances] arithmetic underflow calculating net worth"
            ))?;
        Ok(SignedMoney::new(amount, currency))
    }

    /// All distinct currencies held across every account
    pub fn currencies_held(&self) -> Vec<Currency> {
        let mut currencies: Vec<Currency> = Vec::new();
        for account in &self.accounts {
            let c = account.balance().currency;
            if !currencies.contains(&c) {
                currencies.push(c);
            }
        }
        currencies
    }

    /// Net worth across every currency held — one SignedMoney per currency
    pub fn net_worth_all_currencies(&self) -> Result<Vec<SignedMoney>, SharedError> {
        self.currencies_held()
            .into_iter()
            .map(|c| self.net_worth(c))
            .collect()
    }
}
