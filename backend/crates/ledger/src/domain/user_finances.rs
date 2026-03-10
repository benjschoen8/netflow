use shared::domain::UserId;
use shared::domain::SharedError;
use shared::domain::AggregateRoot;

use crate::domain::account_id::AccountId;
use crate::domain::balance::Balance;
use crate::domain::currency::Currency;
use crate::domain::liability::Liability;
use crate::domain::money::Money;
use crate::domain::financial_account::FinancialAccount;
use crate::domain::cash_account::CashAccount;
use crate::domain::investment_account::InvestmentAccount;
use crate::domain::credit_card::CreditCard;
use crate::domain::loan_account::LoanAccount;
use crate::domain::physical_wallet::PhysicalWallet;
use crate::domain::digital_wallet::DigitalWallet;
use crate::domain::investment::Investment;
use crate::domain::ticker::Ticker;
use crate::domain::events::ledger_events::LedgerEvent;
use crate::domain::events::user_finances_created::UserFinancesCreated;
use crate::domain::events::account_removed::AccountRemoved;
use crate::domain::events::cash_account_opened::CashAccountOpened;
use crate::domain::events::physical_wallet_added::PhysicalWalletAdded;
use crate::domain::events::digital_wallet_added::DigitalWalletAdded;
use crate::domain::events::investment_account_opened::InvestmentAccountOpened;
use crate::domain::events::credit_card_added::CreditCardAdded;
use crate::domain::events::loan_account_opened::LoanAccountOpened;
use crate::domain::events::funds_deposited::FundsDeposited;
use crate::domain::events::funds_withdrawn::FundsWithdrawn;
use crate::domain::events::payment_made::PaymentMade;
use crate::domain::events::minimum_payment_met::MinimumPaymentMet;
use crate::domain::events::debt_settled::DebtSettled;
use crate::domain::events::interest_accrued::InterestAccrued;
use crate::domain::events::cycle_reset::CycleReset;
use crate::domain::events::account_marked_overdue::AccountMarkedOverdue;
use crate::domain::events::account_marked_current::AccountMarkedCurrent;
use crate::domain::events::credit_card_charged::CreditCardCharged;
use crate::domain::events::statement_closed::StatementClosed;
use crate::domain::events::holding_added::HoldingAdded;
use crate::domain::events::holding_removed::HoldingRemoved;

#[derive(Debug, Clone, PartialEq)]
pub struct UserFinances {
    owner_id: UserId,
    accounts: Vec<FinancialAccount>,
    domain_events: Vec<LedgerEvent>,
}

impl AggregateRoot for UserFinances {
    type Event = LedgerEvent;

    fn events(&self) -> &[Self::Event] {
        &self.domain_events
    }

    fn record_event(&mut self, event: Self::Event) {
        self.domain_events.push(event);
    }

    fn pull_events(&mut self) -> Vec<Self::Event> {
        std::mem::take(&mut self.domain_events)
    }
}

impl UserFinances {
    pub fn new() -> Self {
        let owner_id = UserId::create();
        let mut user = Self {
            owner_id,
            accounts: Vec::new(),
            domain_events: Vec::new(),
        };
        user.record_event(UserFinancesCreated { user_id: owner_id }.into());
        user
    }

    pub fn restore(owner_id: UserId, accounts: Vec<FinancialAccount>) -> Self {
        Self { owner_id, accounts, domain_events: Vec::new() }
    }

    // ── Getters ───────────────────────────────────────────────────────────────

    pub fn owner_id(&self) -> UserId { self.owner_id }
    pub fn accounts(&self) -> &[FinancialAccount] { &self.accounts }

    // ── Account management ────────────────────────────────────────────────────

    pub fn add_cash_account(&mut self, account: CashAccount) -> Result<(), SharedError> {
        self.ensure_unique(account.account_id())?;
        self.record_event(CashAccountOpened {
            user_id: self.owner_id,
            account_id: account.account_id(),
            account_name: account.account_name().value().to_string(),
            currency: account.currency(),
        }.into());
        self.accounts.push(account.into());
        Ok(())
    }

    pub fn add_physical_wallet(&mut self, account: PhysicalWallet) -> Result<(), SharedError> {
        self.ensure_unique(account.account_id())?;
        self.record_event(PhysicalWalletAdded {
            user_id: self.owner_id,
            account_id: account.account_id(),
            account_name: account.account_name().value().to_string(),
            currency: account.currency(),
        }.into());
        self.accounts.push(account.into());
        Ok(())
    }

    pub fn add_digital_wallet(&mut self, account: DigitalWallet) -> Result<(), SharedError> {
        self.ensure_unique(account.account_id())?;
        self.record_event(DigitalWalletAdded {
            user_id: self.owner_id,
            account_id: account.account_id(),
            account_name: account.account_name().value().to_string(),
            provider: account.provider().clone(),
            currency: account.currency(),
        }.into());
        self.accounts.push(account.into());
        Ok(())
    }

    pub fn add_investment_account(&mut self, account: InvestmentAccount) -> Result<(), SharedError> {
        self.ensure_unique(account.account_id())?;
        self.record_event(InvestmentAccountOpened {
            user_id: self.owner_id,
            account_id: account.account_id(),
            account_name: account.account_name().value().to_string(),
            currency: account.currency(),
        }.into());
        self.accounts.push(account.into());
        Ok(())
    }

    pub fn add_credit_card(&mut self, account: CreditCard) -> Result<(), SharedError> {
        self.ensure_unique(account.account_id())?;
        self.record_event(CreditCardAdded {
            user_id: self.owner_id,
            account_id: account.account_id(),
            account_name: account.account_name().value().to_string(),
            network: account.network().clone(),
            currency: account.currency(),
            credit_limit: account.credit_limit().clone(),
        }.into());
        self.accounts.push(account.into());
        Ok(())
    }

    pub fn add_loan_account(&mut self, account: LoanAccount) -> Result<(), SharedError> {
        self.ensure_unique(account.account_id())?;
        self.record_event(LoanAccountOpened {
            user_id: self.owner_id,
            account_id: account.account_id(),
            account_name: account.account_name().value().to_string(),
            principal: account.loan().principal().clone(),
            creditor: account.loan().creditor().to_string(),
        }.into());
        self.accounts.push(account.into());
        Ok(())
    }

    pub fn remove_account(&mut self, id: AccountId) -> Result<(), SharedError> {
        let idx = self.find_index(id)?;
        let account_type = self.accounts[idx].account_type();
        self.accounts.remove(idx);
        self.record_event(AccountRemoved {
            user_id: self.owner_id,
            account_id: id,
            account_type,
        }.into());
        Ok(())
    }

    // ── Asset mutations ───────────────────────────────────────────────────────

    pub fn deposit(&mut self, account_id: AccountId, amount: &Money) -> Result<(), SharedError> {
        let idx = self.find_index(account_id)?;
        self.accounts[idx].deposit(amount)?;
        self.record_event(FundsDeposited {
            user_id: self.owner_id,
            account_id,
            amount: amount.clone(),
        }.into());
        Ok(())
    }

    pub fn withdraw(&mut self, account_id: AccountId, amount: &Money) -> Result<(), SharedError> {
        let idx = self.find_index(account_id)?;
        self.accounts[idx].withdraw(amount)?;
        self.record_event(FundsWithdrawn {
            user_id: self.owner_id,
            account_id,
            amount: amount.clone(),
        }.into());
        Ok(())
    }

    // ── Debt mutations ────────────────────────────────────────────────────────

    pub fn make_payment(
        &mut self,
        from_id: AccountId,
        debt_id: AccountId,
        amount: &Liability,
    ) -> Result<(), SharedError> {
        let from_idx = self.find_index(from_id)?;
        let debt_idx = self.find_index(debt_id)?;

        // dry run — validate both sides using read-only arithmetic, no state changed
        let payment_money = Money::new(amount.amount(), amount.currency())?;
        self.accounts[from_idx].balance()?.sub(&payment_money)?;
        self.accounts[debt_idx].outstanding()?.sub(amount)?;

        // both valid — now mutate one at a time, borrow dropped each line
        self.accounts[from_idx].withdraw(&payment_money)?;
        self.accounts[debt_idx].make_payment(amount)?;

        // read outcome — borrow dropped before record_event re-borrows self
        let minimum_met = self.accounts[debt_idx].minimum_payment_paid();
        let is_settled  = self.accounts[debt_idx].is_paid();

        self.record_event(PaymentMade {
            user_id: self.owner_id,
            from_account_id: from_id,
            debt_account_id: debt_id,
            amount: amount.clone(),
        }.into());
        if minimum_met {
            self.record_event(MinimumPaymentMet {
                user_id: self.owner_id,
                account_id: debt_id,
            }.into());
        }
        if is_settled {
            self.record_event(DebtSettled {
                user_id: self.owner_id,
                account_id: debt_id,
            }.into());
        }
        Ok(())
    }

    pub fn accrue_interest(&mut self, account_id: AccountId) -> Result<(), SharedError> {
        let idx = self.find_index(account_id)?;

        // capture before — borrow dropped
        let before = self.accounts[idx].outstanding()?.clone();
        self.accounts[idx].accrue_interest()?;
        let after = self.accounts[idx].outstanding()?.clone();

        let accrued = after.sub(&before)?;
        if !accrued.is_zero() {
            self.record_event(InterestAccrued {
                user_id: self.owner_id,
                account_id,
                amount: accrued,
            }.into());
        }
        Ok(())
    }

    pub fn reset_cycle(&mut self, account_id: AccountId) -> Result<(), SharedError> {
        let idx = self.find_index(account_id)?;
        self.accounts[idx].reset_cycle()?;
        self.record_event(CycleReset {
            user_id: self.owner_id,
            account_id,
        }.into());
        Ok(())
    }

    pub fn mark_overdue(&mut self, account_id: AccountId) -> Result<(), SharedError> {
        let idx = self.find_index(account_id)?;
        self.accounts[idx].mark_overdue()?;
        self.record_event(AccountMarkedOverdue {
            user_id: self.owner_id,
            account_id,
        }.into());
        Ok(())
    }

    pub fn mark_current(&mut self, account_id: AccountId) -> Result<(), SharedError> {
        let idx = self.find_index(account_id)?;
        self.accounts[idx].mark_current()?;
        self.record_event(AccountMarkedCurrent {
            user_id: self.owner_id,
            account_id,
        }.into());
        Ok(())
    }

    // ── Credit card specific ──────────────────────────────────────────────────

    pub fn charge_credit_card(
        &mut self,
        account_id: AccountId,
        amount: &Liability,
    ) -> Result<(), SharedError> {
        let idx = self.find_index(account_id)?;
        self.accounts[idx].charge(amount)?;
        self.record_event(CreditCardCharged {
            user_id: self.owner_id,
            account_id,
            amount: amount.clone(),
        }.into());
        Ok(())
    }

    pub fn close_statement(
        &mut self,
        account_id: AccountId,
        minimum_payment: Option<Liability>,
    ) -> Result<(), SharedError> {
        let idx = self.find_index(account_id)?;
        self.accounts[idx].close_statement(minimum_payment.clone())?;

        // read statement balance after closing — borrow dropped before record_event
        let statement_balance = self.accounts[idx].statement_balance()?;

        self.record_event(StatementClosed {
            user_id: self.owner_id,
            account_id,
            statement_balance,
            minimum_payment,
        }.into());
        Ok(())
    }

    // ── Investment specific ───────────────────────────────────────────────────

    pub fn add_holding(
        &mut self,
        account_id: AccountId,
        investment: Investment,
    ) -> Result<(), SharedError> {
        // build event before investment is moved
        let event = HoldingAdded {
            user_id: self.owner_id,
            account_id,
            ticker: investment.ticker().clone(),
            investment_type: investment.investment_type().clone(),
            quantity: investment.quantity(),
            unit_price: investment.unit_price().clone(),
        };
        let idx = self.find_index(account_id)?;
        self.accounts[idx].add_holding(investment)?;
        self.record_event(event.into());
        Ok(())
    }

    pub fn remove_holding(
        &mut self,
        account_id: AccountId,
        ticker: &Ticker,
    ) -> Result<(), SharedError> {
        let idx = self.find_index(account_id)?;
        self.accounts[idx].remove_holding(ticker)?;
        self.record_event(HoldingRemoved {
            user_id: self.owner_id,
            account_id,
            ticker: ticker.clone(),
        }.into());
        Ok(())
    }

    // ── Filtered views ────────────────────────────────────────────────────────

    pub fn cash_accounts(&self)       -> impl Iterator<Item = &CashAccount>       { self.accounts.iter().filter_map(|a| a.as_cash()) }
    pub fn investment_accounts(&self) -> impl Iterator<Item = &InvestmentAccount> { self.accounts.iter().filter_map(|a| a.as_investment()) }
    pub fn credit_cards(&self)        -> impl Iterator<Item = &CreditCard>        { self.accounts.iter().filter_map(|a| a.as_credit_card()) }
    pub fn loan_accounts(&self)       -> impl Iterator<Item = &LoanAccount>       { self.accounts.iter().filter_map(|a| a.as_loan()) }
    pub fn physical_wallets(&self)    -> impl Iterator<Item = &PhysicalWallet>    { self.accounts.iter().filter_map(|a| a.as_physical_wallet()) }
    pub fn digital_wallets(&self)     -> impl Iterator<Item = &DigitalWallet>     { self.accounts.iter().filter_map(|a| a.as_digital_wallet()) }

    pub fn overdue_accounts(&self) -> impl Iterator<Item = &FinancialAccount> {
        self.accounts.iter().filter(|a| a.is_overdue())
    }

    pub fn find_account(&self, id: AccountId) -> Option<&FinancialAccount> {
        self.accounts.iter().find(|a| a.account_id() == id)
    }

    pub fn find_account_mut(&mut self, id: AccountId) -> Option<&mut FinancialAccount> {
        self.accounts.iter_mut().find(|a| a.account_id() == id)
    }

    // ── Summaries ─────────────────────────────────────────────────────────────

    pub fn total_assets(&self, currency: Currency) -> Result<Money, SharedError> {
        self.accounts
            .iter()
            .filter_map(|a| match a.balance() {
                Balance::Asset(m) if m.currency() == currency => Some(m),
                _ => None,
            })
            .try_fold(Money::zero(currency), |acc, m| acc.add(&m))
    }

    pub fn total_liabilities(&self, currency: Currency) -> Result<Liability, SharedError> {
        self.accounts
            .iter()
            .filter_map(|a| match a.balance() {
                Balance::Debt(l) if l.currency() == currency => Some(l),
                _ => None,
            })
            .try_fold(Liability::zero(currency), |acc, l| acc.add(&l))
    }

    pub fn net_worth(&self, currency: Currency) -> Result<Balance, SharedError> {
        let assets      = self.total_assets(currency)?;
        let liabilities = self.total_liabilities(currency)?;
        assets.cross_sub(&liabilities)
    }

    pub fn currencies_held(&self) -> Vec<Currency> {
        let mut seen: Vec<Currency> = Vec::new();
        for account in &self.accounts {
            let c = account.currency();
            if !seen.contains(&c) { seen.push(c); }
        }
        seen
    }

    pub fn net_worth_all_currencies(&self) -> Result<Vec<Balance>, SharedError> {
        self.currencies_held()
            .into_iter()
            .map(|c| self.net_worth(c))
            .collect()
    }

    // ── Private helpers ───────────────────────────────────────────────────────

    fn ensure_unique(&self, id: AccountId) -> Result<(), SharedError> {
        if self.accounts.iter().any(|a| a.account_id() == id) {
            return Err(SharedError::Operational(
                "[UserFinances] account with this id already exists"
            ));
        }
        Ok(())
    }

    fn find_index(&self, id: AccountId) -> Result<usize, SharedError> {
        self.accounts
            .iter()
            .position(|a| a.account_id() == id)
            .ok_or(SharedError::Operational("[UserFinances] account not found"))
    }
}