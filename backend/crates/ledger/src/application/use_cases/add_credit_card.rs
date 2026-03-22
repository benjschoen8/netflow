use rust_decimal::Decimal;
use shared::domain::AggregateRoot;
use shared::domain::UserId;

use crate::application::error::LedgerError;
use crate::application::parse_helpers::parse_card_network;
use crate::application::ports::{LedgerUnitOfWork, WriteOperation};
use crate::domain::account_id::AccountId;
use crate::domain::account_name::AccountName;
use crate::domain::card_last_four::CardLastFour;
use crate::domain::credit_card::CreditCard;
use crate::domain::currency::Currency;
use crate::domain::expiration_date::ExpirationDate;
use crate::domain::liability::Liability;
use crate::domain::money::Money;
use crate::domain::monthly_day::MonthlyDay;

pub struct AddCreditCardCommand {
    pub owner_id:           UserId,
    pub name:               String,
    pub last_four:          String,
    /// "visa", "mastercard", "amex", "unionpay", "discover", or any other string
    pub network:            String,
    pub expiry_month:       u8,
    pub expiry_year:        u16,
    pub credit_limit:       Decimal,
    pub currency:           Currency,
    pub outstanding:        Decimal,
    pub cash_advance_limit: Option<Decimal>,
    pub statement_day:      u8,
    pub due_day:            u8,
    pub interest_rate:      Option<Decimal>,
}

pub async fn execute(
    uow: &dyn LedgerUnitOfWork,
    cmd: AddCreditCardCommand,
) -> Result<(), LedgerError> {
    let mut finances = uow.load(cmd.owner_id).await?;

    let cash_advance = cmd.cash_advance_limit
        .map(|a| Money::new(a, cmd.currency))
        .transpose()?;

    let card = CreditCard::new(
        AccountId::create(),
        AccountName::new(cmd.name)?,
        CardLastFour::new(cmd.last_four)?,
        parse_card_network(&cmd.network)?,   // String → CardNetwork here
        ExpirationDate::new(cmd.expiry_month, cmd.expiry_year)?,
        Money::new(cmd.credit_limit, cmd.currency)?,
        cash_advance,
        Liability::new(cmd.outstanding, cmd.currency)?,
        MonthlyDay::new(cmd.statement_day)?,
        MonthlyDay::new(cmd.due_day)?,
        cmd.interest_rate,
    )?;

    finances.add_credit_card(card)?;
    let _events = finances.pull_events();
    uow.commit(WriteOperation::new(&finances, vec![])).await
}
