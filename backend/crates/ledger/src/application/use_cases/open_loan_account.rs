use rust_decimal::Decimal;
use chrono::NaiveDate;
use shared::domain::AggregateRoot;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::ports::{LedgerUnitOfWork, WriteOperation};
use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::account_number::AccountNumber;
use crate::domain::bank::Bank;
use crate::domain::currency::Currency;
use crate::domain::liability::Liability;
use crate::domain::loan::Loan;
use crate::domain::loan_account::LoanAccount;
use crate::domain::monthly_day::MonthlyDay;

pub struct OpenLoanAccountCommand {
    pub owner_id:        UserId,
    pub name:            String,
    /// Optional — some informal loans have no account number.
    pub account_number:  Option<String>,
    pub bank:            String,
    pub creditor:        String,
    pub currency:        Currency,
    pub principal:       Decimal,
    /// Annual rate as a percentage, e.g. `3.5`.
    pub interest_rate:   Option<Decimal>,
    /// Day of the month the payment is due.
    pub due_day:         Option<u8>,
    /// Final maturity date.
    pub maturity_date:   Option<NaiveDate>,
    /// Fixed monthly minimum payment, if applicable.
    pub minimum_payment: Option<Decimal>,
}

pub async fn execute(
    uow: &dyn LedgerUnitOfWork,
    cmd: OpenLoanAccountCommand,
) -> Result<(), LedgerError> {
    let mut finances = uow.load(cmd.owner_id).await?;

    let account_number = cmd.account_number
        .map(AccountNumber::new)
        .transpose()?;

    let minimum_payment = cmd.minimum_payment
        .map(|a| Liability::new(a, cmd.currency))
        .transpose()?;

    let due_day = cmd.due_day
        .map(MonthlyDay::new)
        .transpose()?;

    let loan = Loan::new(
        Liability::new(cmd.principal, cmd.currency)?,
        cmd.creditor,
        cmd.interest_rate,
        due_day,
        cmd.maturity_date,
        minimum_payment,
    )?;

    let account = LoanAccount::new(
        AccountId::create(),
        AccountName::new(cmd.name)?,
        account_number,
        Bank::new(cmd.bank)?,
        loan,
    );

    finances.add_loan_account(account)?;
    let _events = finances.pull_events();
    uow.commit(WriteOperation::new(&finances, vec![])).await
}
