//! `LedgerEvent` — the single event envelope for the ledger bounded context.
//!
//! We implement `DomainEvent` manually instead of using `enum_dispatch` because
//! `enum_dispatch` cannot generate cross-crate `From<Variant>` impls when the
//! dispatched trait (`DomainEvent`) lives in a different crate (`shared`).
//! Manual impls are explicit, zero-magic, and compile reliably.

use serde::Serialize;
use shared::domain::{DomainEvent, EventId, Timestamp};

use crate::domain::events::account_marked_current::AccountMarkedCurrent;
use crate::domain::events::account_marked_overdue::AccountMarkedOverdue;
use crate::domain::events::account_removed::AccountRemoved;
use crate::domain::events::cash_account_opened::CashAccountOpened;
use crate::domain::events::credit_card_added::CreditCardAdded;
use crate::domain::events::credit_card_charged::CreditCardCharged;
use crate::domain::events::cycle_reset::CycleReset;
use crate::domain::events::debt_settled::DebtSettled;
use crate::domain::events::digital_wallet_added::DigitalWalletAdded;
use crate::domain::events::funds_deposited::FundsDeposited;
use crate::domain::events::funds_withdrawn::FundsWithdrawn;
use crate::domain::events::holding_added::HoldingAdded;
use crate::domain::events::holding_price_updated::HoldingPriceUpdated;
use crate::domain::events::holding_removed::HoldingRemoved;
use crate::domain::events::interest_accrued::InterestAccrued;
use crate::domain::events::investment_account_opened::InvestmentAccountOpened;
use crate::domain::events::loan_account_opened::LoanAccountOpened;
use crate::domain::events::minimum_payment_met::MinimumPaymentMet;
use crate::domain::events::payment_made::PaymentMade;
use crate::domain::events::physical_wallet_added::PhysicalWalletAdded;
use crate::domain::events::statement_closed::StatementClosed;
use crate::domain::events::temporary_credit_limit_granted::TemporaryCreditLimitGranted;
use crate::domain::events::temporary_credit_limit_revoked::TemporaryCreditLimitRevoked;
use crate::domain::events::user_finances_created::UserFinancesCreated;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum LedgerEvent {
    // aggregate
    UserFinancesCreated(UserFinancesCreated),
    // account lifecycle
    AccountRemoved(AccountRemoved),
    CashAccountOpened(CashAccountOpened),
    PhysicalWalletAdded(PhysicalWalletAdded),
    DigitalWalletAdded(DigitalWalletAdded),
    InvestmentAccountOpened(InvestmentAccountOpened),
    CreditCardAdded(CreditCardAdded),
    LoanAccountOpened(LoanAccountOpened),
    // asset mutations
    FundsDeposited(FundsDeposited),
    FundsWithdrawn(FundsWithdrawn),
    // debt mutations
    PaymentMade(PaymentMade),
    MinimumPaymentMet(MinimumPaymentMet),
    DebtSettled(DebtSettled),
    InterestAccrued(InterestAccrued),
    CycleReset(CycleReset),
    AccountMarkedOverdue(AccountMarkedOverdue),
    AccountMarkedCurrent(AccountMarkedCurrent),
    // credit card specific
    CreditCardCharged(CreditCardCharged),
    StatementClosed(StatementClosed),
    TemporaryCreditLimitGranted(TemporaryCreditLimitGranted),
    TemporaryCreditLimitRevoked(TemporaryCreditLimitRevoked),
    // investment specific
    HoldingAdded(HoldingAdded),
    HoldingRemoved(HoldingRemoved),
    HoldingPriceUpdated(HoldingPriceUpdated),
}

// ── DomainEvent impl — delegate to the inner event ───────────────────────────

impl DomainEvent for LedgerEvent {
    fn event_id(&self) -> EventId {
        match self {
            Self::UserFinancesCreated(e)         => e.event_id(),
            Self::AccountRemoved(e)              => e.event_id(),
            Self::CashAccountOpened(e)           => e.event_id(),
            Self::PhysicalWalletAdded(e)         => e.event_id(),
            Self::DigitalWalletAdded(e)          => e.event_id(),
            Self::InvestmentAccountOpened(e)     => e.event_id(),
            Self::CreditCardAdded(e)             => e.event_id(),
            Self::LoanAccountOpened(e)           => e.event_id(),
            Self::FundsDeposited(e)              => e.event_id(),
            Self::FundsWithdrawn(e)              => e.event_id(),
            Self::PaymentMade(e)                 => e.event_id(),
            Self::MinimumPaymentMet(e)           => e.event_id(),
            Self::DebtSettled(e)                 => e.event_id(),
            Self::InterestAccrued(e)             => e.event_id(),
            Self::CycleReset(e)                  => e.event_id(),
            Self::AccountMarkedOverdue(e)        => e.event_id(),
            Self::AccountMarkedCurrent(e)        => e.event_id(),
            Self::CreditCardCharged(e)           => e.event_id(),
            Self::StatementClosed(e)             => e.event_id(),
            Self::TemporaryCreditLimitGranted(e) => e.event_id(),
            Self::TemporaryCreditLimitRevoked(e) => e.event_id(),
            Self::HoldingAdded(e)                => e.event_id(),
            Self::HoldingRemoved(e)              => e.event_id(),
            Self::HoldingPriceUpdated(e)         => e.event_id(),
        }
    }

    fn occurred_on(&self) -> Timestamp {
        match self {
            Self::UserFinancesCreated(e)         => e.occurred_on(),
            Self::AccountRemoved(e)              => e.occurred_on(),
            Self::CashAccountOpened(e)           => e.occurred_on(),
            Self::PhysicalWalletAdded(e)         => e.occurred_on(),
            Self::DigitalWalletAdded(e)          => e.occurred_on(),
            Self::InvestmentAccountOpened(e)     => e.occurred_on(),
            Self::CreditCardAdded(e)             => e.occurred_on(),
            Self::LoanAccountOpened(e)           => e.occurred_on(),
            Self::FundsDeposited(e)              => e.occurred_on(),
            Self::FundsWithdrawn(e)              => e.occurred_on(),
            Self::PaymentMade(e)                 => e.occurred_on(),
            Self::MinimumPaymentMet(e)           => e.occurred_on(),
            Self::DebtSettled(e)                 => e.occurred_on(),
            Self::InterestAccrued(e)             => e.occurred_on(),
            Self::CycleReset(e)                  => e.occurred_on(),
            Self::AccountMarkedOverdue(e)        => e.occurred_on(),
            Self::AccountMarkedCurrent(e)        => e.occurred_on(),
            Self::CreditCardCharged(e)           => e.occurred_on(),
            Self::StatementClosed(e)             => e.occurred_on(),
            Self::TemporaryCreditLimitGranted(e) => e.occurred_on(),
            Self::TemporaryCreditLimitRevoked(e) => e.occurred_on(),
            Self::HoldingAdded(e)                => e.occurred_on(),
            Self::HoldingRemoved(e)              => e.occurred_on(),
            Self::HoldingPriceUpdated(e)         => e.occurred_on(),
        }
    }

    fn event_type(&self) -> &'static str {
        match self {
            Self::UserFinancesCreated(e)         => e.event_type(),
            Self::AccountRemoved(e)              => e.event_type(),
            Self::CashAccountOpened(e)           => e.event_type(),
            Self::PhysicalWalletAdded(e)         => e.event_type(),
            Self::DigitalWalletAdded(e)          => e.event_type(),
            Self::InvestmentAccountOpened(e)     => e.event_type(),
            Self::CreditCardAdded(e)             => e.event_type(),
            Self::LoanAccountOpened(e)           => e.event_type(),
            Self::FundsDeposited(e)              => e.event_type(),
            Self::FundsWithdrawn(e)              => e.event_type(),
            Self::PaymentMade(e)                 => e.event_type(),
            Self::MinimumPaymentMet(e)           => e.event_type(),
            Self::DebtSettled(e)                 => e.event_type(),
            Self::InterestAccrued(e)             => e.event_type(),
            Self::CycleReset(e)                  => e.event_type(),
            Self::AccountMarkedOverdue(e)        => e.event_type(),
            Self::AccountMarkedCurrent(e)        => e.event_type(),
            Self::CreditCardCharged(e)           => e.event_type(),
            Self::StatementClosed(e)             => e.event_type(),
            Self::TemporaryCreditLimitGranted(e) => e.event_type(),
            Self::TemporaryCreditLimitRevoked(e) => e.event_type(),
            Self::HoldingAdded(e)                => e.event_type(),
            Self::HoldingRemoved(e)              => e.event_type(),
            Self::HoldingPriceUpdated(e)         => e.event_type(),
        }
    }

    fn event_version(&self) -> &'static str { "1.0" }

    fn domain(&self) -> &'static str { "ledger" }

    fn service(&self) -> &'static str { "ledger" }
}

// ── From impls — one per variant, no macros, no magic ────────────────────────

impl From<UserFinancesCreated>         for LedgerEvent { fn from(e: UserFinancesCreated)         -> Self { Self::UserFinancesCreated(e) } }
impl From<AccountRemoved>              for LedgerEvent { fn from(e: AccountRemoved)              -> Self { Self::AccountRemoved(e) } }
impl From<CashAccountOpened>           for LedgerEvent { fn from(e: CashAccountOpened)           -> Self { Self::CashAccountOpened(e) } }
impl From<PhysicalWalletAdded>         for LedgerEvent { fn from(e: PhysicalWalletAdded)         -> Self { Self::PhysicalWalletAdded(e) } }
impl From<DigitalWalletAdded>          for LedgerEvent { fn from(e: DigitalWalletAdded)          -> Self { Self::DigitalWalletAdded(e) } }
impl From<InvestmentAccountOpened>     for LedgerEvent { fn from(e: InvestmentAccountOpened)     -> Self { Self::InvestmentAccountOpened(e) } }
impl From<CreditCardAdded>             for LedgerEvent { fn from(e: CreditCardAdded)             -> Self { Self::CreditCardAdded(e) } }
impl From<LoanAccountOpened>           for LedgerEvent { fn from(e: LoanAccountOpened)           -> Self { Self::LoanAccountOpened(e) } }
impl From<FundsDeposited>              for LedgerEvent { fn from(e: FundsDeposited)              -> Self { Self::FundsDeposited(e) } }
impl From<FundsWithdrawn>              for LedgerEvent { fn from(e: FundsWithdrawn)              -> Self { Self::FundsWithdrawn(e) } }
impl From<PaymentMade>                 for LedgerEvent { fn from(e: PaymentMade)                 -> Self { Self::PaymentMade(e) } }
impl From<MinimumPaymentMet>           for LedgerEvent { fn from(e: MinimumPaymentMet)           -> Self { Self::MinimumPaymentMet(e) } }
impl From<DebtSettled>                 for LedgerEvent { fn from(e: DebtSettled)                 -> Self { Self::DebtSettled(e) } }
impl From<InterestAccrued>             for LedgerEvent { fn from(e: InterestAccrued)             -> Self { Self::InterestAccrued(e) } }
impl From<CycleReset>                  for LedgerEvent { fn from(e: CycleReset)                  -> Self { Self::CycleReset(e) } }
impl From<AccountMarkedOverdue>        for LedgerEvent { fn from(e: AccountMarkedOverdue)        -> Self { Self::AccountMarkedOverdue(e) } }
impl From<AccountMarkedCurrent>        for LedgerEvent { fn from(e: AccountMarkedCurrent)        -> Self { Self::AccountMarkedCurrent(e) } }
impl From<CreditCardCharged>           for LedgerEvent { fn from(e: CreditCardCharged)           -> Self { Self::CreditCardCharged(e) } }
impl From<StatementClosed>             for LedgerEvent { fn from(e: StatementClosed)             -> Self { Self::StatementClosed(e) } }
impl From<TemporaryCreditLimitGranted> for LedgerEvent { fn from(e: TemporaryCreditLimitGranted) -> Self { Self::TemporaryCreditLimitGranted(e) } }
impl From<TemporaryCreditLimitRevoked> for LedgerEvent { fn from(e: TemporaryCreditLimitRevoked) -> Self { Self::TemporaryCreditLimitRevoked(e) } }
impl From<HoldingAdded>                for LedgerEvent { fn from(e: HoldingAdded)                -> Self { Self::HoldingAdded(e) } }
impl From<HoldingRemoved>              for LedgerEvent { fn from(e: HoldingRemoved)              -> Self { Self::HoldingRemoved(e) } }
impl From<HoldingPriceUpdated>         for LedgerEvent { fn from(e: HoldingPriceUpdated)         -> Self { Self::HoldingPriceUpdated(e) } }
