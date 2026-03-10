use crate::domain::account_id::AccountId;
use crate::domain::balance::Balance;
use crate::domain::currency::Currency;
use crate::domain::financial_entry::FinancialEntry;
use crate::domain::cash_account::CashAccount;
use crate::domain::credit_card::CreditCard;
use crate::domain::digital_wallet::DigitalWallet;
use crate::domain::investment_account::InvestmentAccount;
use crate::domain::loan_account::LoanAccount;
use crate::domain::physical_wallet::PhysicalWallet;

#[derive(Debug, Clone, PartialEq)]
pub enum FinancialAccount {
    Cash(CashAccount),
    Investment(InvestmentAccount),
    CreditCard(CreditCard),
    Loan(LoanAccount),
    PhysicalWallet(PhysicalWallet),
    DigitalWallet(DigitalWallet),
}

impl FinancialAccount {
    pub fn account_id(&self) -> AccountId {
        match self {
            Self::Cash(a)           => a.account_id(),
            Self::Investment(a)     => a.account_id(),
            Self::CreditCard(a)     => a.account_id(),
            Self::Loan(a)           => a.account_id(),
            Self::PhysicalWallet(a) => a.account_id(),
            Self::DigitalWallet(a)  => a.account_id(),
        }
    }

    pub fn account_type(&self) -> &'static str {
        match self {
            Self::Cash(a)           => a.account_type(),
            Self::Investment(a)     => a.account_type(),
            Self::CreditCard(a)     => a.account_type(),
            Self::Loan(a)           => a.account_type(),
            Self::PhysicalWallet(a) => a.account_type(),
            Self::DigitalWallet(a)  => a.account_type(),
        }
    }

    pub fn currency(&self) -> Currency {
        match self {
            Self::Cash(a)           => a.currency(),
            Self::Investment(a)     => a.currency(),
            Self::CreditCard(a)     => a.currency(),
            Self::Loan(a)           => a.currency(),
            Self::PhysicalWallet(a) => a.currency(),
            Self::DigitalWallet(a)  => a.currency(),
        }
    }

    pub fn balance(&self) -> Balance {
        match self {
            Self::Cash(a)           => Balance::Asset(a.balance().clone()),
            Self::Investment(a)     => Balance::Asset(a.balance().clone()),
            Self::PhysicalWallet(a) => Balance::Asset(a.balance().clone()),
            Self::DigitalWallet(a)  => Balance::Asset(a.balance().clone()),
            Self::CreditCard(a)     => Balance::Debt(a.outstanding().clone()),
            Self::Loan(a)           => Balance::Debt(a.outstanding().clone()),
        }
    }

    pub fn is_asset(&self) -> bool {
        matches!(self,
            Self::Cash(_) | Self::Investment(_) |
            Self::PhysicalWallet(_) | Self::DigitalWallet(_)
        )
    }

    pub fn is_overdue(&self) -> bool {
        match self {
            Self::CreditCard(c) => c.is_overdue(),
            Self::Loan(l)       => l.is_overdue(),
            _                   => false,
        }
    }

    // ── Immutable downcasts ───────────────────────────────────────────────────

    pub fn as_cash(&self) -> Option<&CashAccount> {
        match self { Self::Cash(a) => Some(a), _ => None }
    }
    pub fn as_investment(&self) -> Option<&InvestmentAccount> {
        match self { Self::Investment(a) => Some(a), _ => None }
    }
    pub fn as_credit_card(&self) -> Option<&CreditCard> {
        match self { Self::CreditCard(a) => Some(a), _ => None }
    }
    pub fn as_loan(&self) -> Option<&LoanAccount> {
        match self { Self::Loan(a) => Some(a), _ => None }
    }
    pub fn as_physical_wallet(&self) -> Option<&PhysicalWallet> {
        match self { Self::PhysicalWallet(a) => Some(a), _ => None }
    }
    pub fn as_digital_wallet(&self) -> Option<&DigitalWallet> {
        match self { Self::DigitalWallet(a) => Some(a), _ => None }
    }

    // ── Mutable downcasts ─────────────────────────────────────────────────────

    pub fn as_cash_mut(&mut self) -> Option<&mut CashAccount> {
        match self { Self::Cash(a) => Some(a), _ => None }
    }
    pub fn as_investment_mut(&mut self) -> Option<&mut InvestmentAccount> {
        match self { Self::Investment(a) => Some(a), _ => None }
    }
    pub fn as_credit_card_mut(&mut self) -> Option<&mut CreditCard> {
        match self { Self::CreditCard(a) => Some(a), _ => None }
    }
    pub fn as_loan_mut(&mut self) -> Option<&mut LoanAccount> {
        match self { Self::Loan(a) => Some(a), _ => None }
    }
    pub fn as_physical_wallet_mut(&mut self) -> Option<&mut PhysicalWallet> {
        match self { Self::PhysicalWallet(a) => Some(a), _ => None }
    }
    pub fn as_digital_wallet_mut(&mut self) -> Option<&mut DigitalWallet> {
        match self { Self::DigitalWallet(a) => Some(a), _ => None }
    }
}

// ── From impls ────────────────────────────────────────────────────────────────

impl From<CashAccount>       for FinancialAccount { fn from(a: CashAccount)       -> Self { Self::Cash(a) } }
impl From<InvestmentAccount> for FinancialAccount { fn from(a: InvestmentAccount) -> Self { Self::Investment(a) } }
impl From<CreditCard>        for FinancialAccount { fn from(a: CreditCard)        -> Self { Self::CreditCard(a) } }
impl From<LoanAccount>       for FinancialAccount { fn from(a: LoanAccount)       -> Self { Self::Loan(a) } }
impl From<PhysicalWallet>    for FinancialAccount { fn from(a: PhysicalWallet)    -> Self { Self::PhysicalWallet(a) } }
impl From<DigitalWallet>     for FinancialAccount { fn from(a: DigitalWallet)     -> Self { Self::DigitalWallet(a) } }
