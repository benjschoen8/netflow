use shared::domain::SharedError;

use crate::domain::account_id::AccountId;
use crate::domain::balance::Balance;
use crate::domain::currency::Currency;
use crate::domain::financial_entry::FinancialEntry;
use crate::domain::asset_account::AssetAccount;
use crate::domain::debt_account::DebtAccount;
use crate::domain::revolving_credit::RevolvingCredit;
use crate::domain::cash_account::CashAccount;
use crate::domain::credit_card::CreditCard;
use crate::domain::digital_wallet::DigitalWallet;
use crate::domain::investment_account::InvestmentAccount;
use crate::domain::investment::Investment;
use crate::domain::ticker::Ticker;
use crate::domain::loan_account::LoanAccount;
use crate::domain::liability::Liability;
use crate::domain::money::Money;
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

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Returned when a method that only makes sense for one account kind is called
/// on the wrong variant (e.g., calling `outstanding()` on a cash account).
fn wrong_type(method: &'static str) -> SharedError {
    SharedError::Operational(method)
}

impl FinancialAccount {
    // ── FinancialEntry forwarding ─────────────────────────────────────────────

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

    pub fn account_name_str(&self) -> &str {
        match self {
            Self::Cash(a)           => a.account_name().value(),
            Self::Investment(a)     => a.account_name().value(),
            Self::CreditCard(a)     => a.account_name().value(),
            Self::Loan(a)           => a.account_name().value(),
            Self::PhysicalWallet(a) => a.account_name().value(),
            Self::DigitalWallet(a)  => a.account_name().value(),
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

    // ── Balance summary ───────────────────────────────────────────────────────

    /// Returns the high-level `Balance` enum (asset or debt) for summary views.
    pub fn balance_summary(&self) -> Balance {
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
        matches!(
            self,
            Self::Cash(_) | Self::Investment(_) |
            Self::PhysicalWallet(_) | Self::DigitalWallet(_)
        )
    }

    // ── AssetAccount dispatch ─────────────────────────────────────────────────

    /// Returns the asset balance.
    /// Errors with `Operational` if called on a debt account.
    pub fn asset_balance(&self) -> Result<&Money, SharedError> {
        match self {
            Self::Cash(a)           => Ok(a.balance()),
            Self::Investment(a)     => Ok(a.balance()),
            Self::PhysicalWallet(a) => Ok(a.balance()),
            Self::DigitalWallet(a)  => Ok(a.balance()),
            _ => Err(wrong_type("[FinancialAccount] asset_balance called on a debt account")),
        }
    }

    pub fn deposit(&mut self, amount: &Money) -> Result<(), SharedError> {
        match self {
            Self::Cash(a)           => a.deposit(amount),
            Self::Investment(a)     => a.deposit(amount),
            Self::PhysicalWallet(a) => a.deposit(amount),
            Self::DigitalWallet(a)  => a.deposit(amount),
            _ => Err(wrong_type("[FinancialAccount] deposit called on a debt account")),
        }
    }

    pub fn withdraw(&mut self, amount: &Money) -> Result<(), SharedError> {
        match self {
            Self::Cash(a)           => a.withdraw(amount),
            Self::Investment(a)     => a.withdraw(amount),
            Self::PhysicalWallet(a) => a.withdraw(amount),
            Self::DigitalWallet(a)  => a.withdraw(amount),
            _ => Err(wrong_type("[FinancialAccount] withdraw called on a debt account")),
        }
    }

    // ── DebtAccount dispatch ──────────────────────────────────────────────────

    /// Returns the outstanding liability.
    /// Errors if called on an asset account.
    pub fn outstanding(&self) -> Result<&Liability, SharedError> {
        match self {
            Self::CreditCard(a) => Ok(a.outstanding()),
            Self::Loan(a)       => Ok(a.outstanding()),
            _ => Err(wrong_type("[FinancialAccount] outstanding called on an asset account")),
        }
    }

    pub fn make_payment(&mut self, amount: &Liability) -> Result<(), SharedError> {
        match self {
            Self::CreditCard(a) => a.make_payment(amount),
            Self::Loan(a)       => a.make_payment(amount),
            _ => Err(wrong_type("[FinancialAccount] make_payment called on an asset account")),
        }
    }

    pub fn accrue_interest(&mut self) -> Result<(), SharedError> {
        match self {
            Self::CreditCard(a) => a.accrue_interest(),
            Self::Loan(a)       => a.accrue_interest(),
            _ => Err(wrong_type("[FinancialAccount] accrue_interest called on an asset account")),
        }
    }

    pub fn reset_cycle(&mut self) -> Result<(), SharedError> {
        match self {
            Self::CreditCard(a) => { a.reset_cycle(); Ok(()) },
            Self::Loan(a)       => { a.reset_cycle(); Ok(()) },
            _ => Err(wrong_type("[FinancialAccount] reset_cycle called on an asset account")),
        }
    }

    pub fn mark_overdue(&mut self) -> Result<(), SharedError> {
        match self {
            Self::CreditCard(a) => { a.mark_overdue(); Ok(()) },
            Self::Loan(a)       => { a.mark_overdue(); Ok(()) },
            _ => Err(wrong_type("[FinancialAccount] mark_overdue called on an asset account")),
        }
    }

    pub fn mark_current(&mut self) -> Result<(), SharedError> {
        match self {
            Self::CreditCard(a) => { a.mark_current(); Ok(()) },
            Self::Loan(a)       => { a.mark_current(); Ok(()) },
            _ => Err(wrong_type("[FinancialAccount] mark_current called on an asset account")),
        }
    }

    pub fn minimum_payment_paid(&self) -> bool {
        match self {
            Self::CreditCard(a) => a.minimum_payment_paid(),
            Self::Loan(a)       => a.minimum_payment_paid(),
            _                   => false,
        }
    }

    pub fn is_paid(&self) -> bool {
        match self {
            Self::CreditCard(a) => a.is_paid(),
            Self::Loan(a)       => a.is_paid(),
            _                   => true, // assets are never "owed"
        }
    }

    pub fn is_overdue(&self) -> bool {
        match self {
            Self::CreditCard(a) => a.is_overdue(),
            Self::Loan(a)       => a.is_overdue(),
            _                   => false,
        }
    }

    // ── RevolvingCredit dispatch ──────────────────────────────────────────────

    /// Charge an amount to a credit card.
    /// Errors if called on any non-revolving-credit account.
    pub fn charge(&mut self, amount: &Liability) -> Result<(), SharedError> {
        match self {
            Self::CreditCard(a) => a.charge(amount),
            _ => Err(wrong_type("[FinancialAccount] charge called on a non-credit-card account")),
        }
    }

    /// Close the billing statement on a credit card.
    pub fn close_statement(
        &mut self,
        minimum_payment: Option<Liability>,
    ) -> Result<(), SharedError> {
        match self {
            Self::CreditCard(a) => { a.close_statement(minimum_payment); Ok(()) },
            _ => Err(wrong_type("[FinancialAccount] close_statement called on a non-credit-card account")),
        }
    }

    /// Read the last closed statement balance.
    pub fn statement_balance(&self) -> Result<Liability, SharedError> {
        match self {
            Self::CreditCard(a) => a
                .statement_balance()
                .cloned()
                .ok_or(wrong_type("[FinancialAccount] no statement has been closed yet")),
            _ => Err(wrong_type("[FinancialAccount] statement_balance called on a non-credit-card account")),
        }
    }

    // ── Investment dispatch ───────────────────────────────────────────────────

    pub fn add_holding(&mut self, investment: Investment) -> Result<(), SharedError> {
        match self {
            Self::Investment(a) => a.add_holding(investment),
            _ => Err(wrong_type("[FinancialAccount] add_holding called on a non-investment account")),
        }
    }

    pub fn remove_holding(&mut self, ticker: &Ticker) -> Result<(), SharedError> {
        match self {
            Self::Investment(a) => a.remove_holding(ticker),
            _ => Err(wrong_type("[FinancialAccount] remove_holding called on a non-investment account")),
        }
    }

    pub fn update_holding_price(
        &mut self,
        ticker: &Ticker,
        new_price: Money,
    ) -> Result<(), SharedError> {
        match self {
            Self::Investment(a) => a.update_holding_price(ticker, new_price),
            _ => Err(wrong_type("[FinancialAccount] update_holding_price called on a non-investment account")),
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
