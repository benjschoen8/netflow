use crate::domain::account_id::AccountId;
use crate::domain::money::Money;
use super::account::Account;
use super::cash_account::CashAccount;
use super::investment_account::InvestmentAccount;
use super::credit_card::CreditCard;
use super::liability_account::LiabilityAccount;
use super::physical_wallet::PhysicalWallet;
use super::digital_wallet::DigitalWallet;

#[derive(Debug, Clone, PartialEq)]
pub enum FinancialAccount {
    Cash(CashAccount),
    Investment(InvestmentAccount),
    CreditCard(CreditCard),
    Liability(LiabilityAccount),
    PhysicalWallet(PhysicalWallet),
    DigitalWallet(DigitalWallet),
}

impl FinancialAccount {
    pub fn account_id(&self) -> AccountId {
        match self {
            Self::Cash(a)           => a.account_id(),
            Self::Investment(a)     => a.account_id(),
            Self::CreditCard(a)     => a.account_id(),
            Self::Liability(a)      => a.account_id(),
            Self::PhysicalWallet(a) => a.account_id(),
            Self::DigitalWallet(a)  => a.account_id(),
        }
    }

    pub fn balance(&self) -> &Money {
        match self {
            Self::Cash(a)           => a.balance(),
            Self::Investment(a)     => a.balance(),
            Self::CreditCard(a)     => a.balance(),
            Self::Liability(a)      => a.balance(),
            Self::PhysicalWallet(a) => a.balance(),
            Self::DigitalWallet(a)  => a.balance(),
        }
    }

    pub fn is_asset(&self) -> bool {
        match self {
            Self::Cash(a)           => a.is_asset(),
            Self::Investment(a)     => a.is_asset(),
            Self::CreditCard(a)     => a.is_asset(),
            Self::Liability(a)      => a.is_asset(),
            Self::PhysicalWallet(a) => a.is_asset(),
            Self::DigitalWallet(a)  => a.is_asset(),
        }
    }

    pub fn account_type(&self) -> &'static str {
        match self {
            Self::Cash(a)           => a.account_type(),
            Self::Investment(a)     => a.account_type(),
            Self::CreditCard(a)     => a.account_type(),
            Self::Liability(a)      => a.account_type(),
            Self::PhysicalWallet(a) => a.account_type(),
            Self::DigitalWallet(a)  => a.account_type(),
        }
    }

    pub fn is_overdue(&self) -> bool {
        match self {
            Self::CreditCard(c) => c.is_overdue(),
            Self::Liability(l)  => l.liability().is_overdue(),
            _ => false,
        }
    }

    // ── Downcasts ─────────────────────────────────────────────────────────────

    pub fn as_cash(&self) -> Option<&CashAccount> {
        match self { Self::Cash(a) => Some(a), _ => None }
    }

    pub fn as_investment(&self) -> Option<&InvestmentAccount> {
        match self { Self::Investment(a) => Some(a), _ => None }
    }

    pub fn as_credit_card(&self) -> Option<&CreditCard> {
        match self { Self::CreditCard(a) => Some(a), _ => None }
    }

    pub fn as_liability(&self) -> Option<&LiabilityAccount> {
        match self { Self::Liability(a) => Some(a), _ => None }
    }

    pub fn as_physical_wallet(&self) -> Option<&PhysicalWallet> {
        match self { Self::PhysicalWallet(a) => Some(a), _ => None }
    }

    pub fn as_digital_wallet(&self) -> Option<&DigitalWallet> {
        match self { Self::DigitalWallet(a) => Some(a), _ => None }
    }

    pub fn as_cash_mut(&mut self) -> Option<&mut CashAccount> {
        match self { Self::Cash(a) => Some(a), _ => None }
    }

    pub fn as_investment_mut(&mut self) -> Option<&mut InvestmentAccount> {
        match self { Self::Investment(a) => Some(a), _ => None }
    }

    pub fn as_credit_card_mut(&mut self) -> Option<&mut CreditCard> {
        match self { Self::CreditCard(a) => Some(a), _ => None }
    }

    pub fn as_liability_mut(&mut self) -> Option<&mut LiabilityAccount> {
        match self { Self::Liability(a) => Some(a), _ => None }
    }

    pub fn as_physical_wallet_mut(&mut self) -> Option<&mut PhysicalWallet> {
        match self { Self::PhysicalWallet(a) => Some(a), _ => None }
    }

    pub fn as_digital_wallet_mut(&mut self) -> Option<&mut DigitalWallet> {
        match self { Self::DigitalWallet(a) => Some(a), _ => None }
    }
}

// ── From impls ────────────────────────────────────────────────────────────────

impl From<CashAccount> for FinancialAccount {
    fn from(a: CashAccount) -> Self { Self::Cash(a) }
}

impl From<InvestmentAccount> for FinancialAccount {
    fn from(a: InvestmentAccount) -> Self { Self::Investment(a) }
}

impl From<CreditCard> for FinancialAccount {
    fn from(a: CreditCard) -> Self { Self::CreditCard(a) }
}

impl From<LiabilityAccount> for FinancialAccount {
    fn from(a: LiabilityAccount) -> Self { Self::Liability(a) }
}

impl From<PhysicalWallet> for FinancialAccount {
    fn from(a: PhysicalWallet) -> Self { Self::PhysicalWallet(a) }
}

impl From<DigitalWallet> for FinancialAccount {
    fn from(a: DigitalWallet) -> Self { Self::DigitalWallet(a) }
}
