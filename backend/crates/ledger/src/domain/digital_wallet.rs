use shared::domain::SharedError;
use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::currency::Currency;
use crate::domain::money::Money;
use crate::domain::financial_entry::FinancialEntry;
use crate::domain::asset_account::AssetAccount;
use crate::domain::digital_wallet_provider::DigitalWalletProvider;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderAccountId(String);

impl ProviderAccountId {
    pub fn new(val: String) -> Result<Self, SharedError> {
        let trimmed = val.trim().to_string();
        if trimmed.is_empty() {
            return Err(SharedError::Empty("[ProviderAccountId] cannot be empty"));
        }
        Ok(Self(trimmed))
    }

    pub fn value(&self) -> &str { &self.0 }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DigitalWallet {
    account_id: AccountId,
    account_name: AccountName,
    provider: DigitalWalletProvider,
    provider_account_id: ProviderAccountId,
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
}

impl FinancialEntry for DigitalWallet {
    fn account_id(&self) -> AccountId { self.account_id }
    fn account_name(&self) -> &AccountName { &self.account_name }
    fn account_type(&self) -> &'static str { "digital_wallet" }
    fn currency(&self) -> Currency { self.balance.currency() }
}

impl AssetAccount for DigitalWallet {
    fn balance(&self) -> &Money { &self.balance }

    fn deposit(&mut self, amount: &Money) -> Result<(), SharedError> {
        self.balance = self.balance.add(amount)?;
        Ok(())
    }

    fn withdraw(&mut self, amount: &Money) -> Result<(), SharedError> {
        self.balance = self.balance.sub(amount)?;
        Ok(())
    }
}
