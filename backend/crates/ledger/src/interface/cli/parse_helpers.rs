//! CLI-specific parsing: date strings → chrono types.
//! String→domain-enum conversions live in `application::parse_helpers`
//! so they are reachable by use cases without depending on the interface layer.

use crate::application::error::LedgerError;

pub fn parse_naive_date(s: &str) -> Result<chrono::NaiveDate, LedgerError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| LedgerError::Validation(
            format!("Invalid date '{}'. Expected YYYY-MM-DD.", s)
        ))
}
