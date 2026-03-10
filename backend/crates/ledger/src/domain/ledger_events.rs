use enum_dispatch::enum_dispatch;
use serde::Serialize;
use shared::domain::DomainEvent;

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
use crate::domain::events::temporary_credit_limit_granted::TemporaryCreditLimitGranted;
use crate::domain::events::temporary_credit_limit_revoked::TemporaryCreditLimitRevoked;
use crate::domain::events::holding_added::HoldingAdded;
use crate::domain::events::holding_removed::HoldingRemoved;
use crate::domain::events::holding_price_updated::HoldingPriceUpdated;

#[enum_dispatch(DomainEvent)]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum LedgerEvent {
    // aggregate
    UserFinancesCreated,

    // account lifecycle
    AccountRemoved,
    CashAccountOpened,
    PhysicalWalletAdded,
    DigitalWalletAdded,
    InvestmentAccountOpened,
    CreditCardAdded,
    LoanAccountOpened,

    // asset mutations
    FundsDeposited,
    FundsWithdrawn,

    // debt mutations
    PaymentMade,
    MinimumPaymentMet,
    DebtSettled,
    InterestAccrued,
    CycleReset,
    AccountMarkedOverdue,
    AccountMarkedCurrent,

    // credit card specific
    CreditCardCharged,
    StatementClosed,
    TemporaryCreditLimitGranted,
    TemporaryCreditLimitRevoked,

    // investment specific
    HoldingAdded,
    HoldingRemoved,
    HoldingPriceUpdated,
}
