use serde::{Deserialize, Serialize};

use crate::domain::money::Money;
use crate::domain::liability::Liability;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Balance {
    Asset(Money),
    Debt(Liability),
}
