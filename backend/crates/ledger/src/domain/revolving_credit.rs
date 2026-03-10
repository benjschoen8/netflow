use shared::domain::SharedError;

use crate::domain::debt_account::DebtAccount;
use crate::domain::liability::Liability;
use crate::domain::money::Money;

pub trait RevolvingCredit: DebtAccount {
    fn credit_limit(&self) -> &Money;
    fn available_credit(&self) -> Result<Money, SharedError>;
    fn utilisation_percent(&self) -> rust_decimal::Decimal;

    fn charge(&mut self, amount: &Liability) -> Result<(), SharedError>;

    fn close_statement(&mut self, minimum_payment: Option<Liability>);
}
