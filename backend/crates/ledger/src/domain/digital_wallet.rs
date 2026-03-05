use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::money::Money;
use crate::domain::digital_wallet_provider::DigitalWalletProvider;
use super::account::Account;
use shared::domain::SharedError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderAccountId(String);

impl ProviderAccountId {
    pub fn new(val: String) -> Result<Self, SharedError> {
        let trimmed = val.trim().to_string();
        if trimmed.is_empty() {
            return Err(SharedError::Empty(
                "[ProviderAccountId] cannot be empty"
            ));
        }
        Ok(Self(trimmed))
    }

    pub fn value(&self) -> &str { &self.0 }
}

/// A digital wallet account — LINE Pay, Apple Pay, JKO Pay, etc.
/// One currency per wallet — user can have multiple digital wallets.
#[derive(Debug, Clone, PartialEq)]
pub struct DigitalWallet {
    account_id: AccountId,
    account_name: AccountName,

    /// Which platform this wallet belongs to
    provider: DigitalWalletProvider,

    /// The account identifier on the provider's system
    provider_account_id: ProviderAccountId,

    /// Stored value balance
    balance: Money,
}

impl DigitalWallet {
    pub fn new(
        account_id: AccountId,
        account_name: AccountName,
        provider: DigitalWalletProvider,
        provider_account_id: ProviderAccountId,
        balance: Money,
    ) -> Self {
        Self { account_id, account_name, provider, provider_account_id, balance }
    }

    pub fn provider(&self) -> &DigitalWalletProvider { &self.provider }
    pub fn provider_account_id(&self) -> &ProviderAccountId { &self.provider_account_id }

    /// Top up the wallet balance
    pub fn top_up(&mut self, amount: Money) -> Result<(), SharedError> {
        self.balance = self.balance.add(&amount)?;
        Ok(())
    }

    /// Spend from the wallet balance
    pub fn spend(&mut self, amount: Money) -> Result<(), SharedError> {
        self.balance = self.balance.sub(&amount)?;
        Ok(())
    }
}

impl Account for DigitalWallet {
    fn account_id(&self) -> AccountId { self.account_id }
    fn account_name(&self) -> &AccountName { &self.account_name }
    fn balance(&self) -> &Money { &self.balance }
    fn account_type(&self) -> &'static str { "digital_wallet" }
    fn is_asset(&self) -> bool { true }
}
