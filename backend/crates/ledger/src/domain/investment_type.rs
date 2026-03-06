use serde::{ Serialize, Deserialize };

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvestmentType {
    Stock,
    Etf,
    MutualFund,
    Bond,
    Crypto,
    Other(String),
}