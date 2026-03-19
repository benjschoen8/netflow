//! String → domain value-object converters shared across use cases.
//! These live in the application layer so both the use cases and the
//! interface layer can use them without either depending on the other.

use crate::application::error::LedgerError;
use crate::domain::card_network::CardNetwork;
use crate::domain::digital_wallet_provider::DigitalWalletProvider;
use crate::domain::investment_type::InvestmentType;

pub fn parse_card_network(s: &str) -> Result<CardNetwork, LedgerError> {
    match s.to_lowercase().as_str() {
        "visa"                     => Ok(CardNetwork::Visa),
        "mastercard" | "mc"        => Ok(CardNetwork::Mastercard),
        "amex" | "americanexpress" => Ok(CardNetwork::AmericanExpress),
        "unionpay" | "cup"         => Ok(CardNetwork::UnionPay),
        "discover"                 => Ok(CardNetwork::Discover),
        other                      => Ok(CardNetwork::Other(other.to_string())),
    }
}

pub fn parse_wallet_provider(s: &str) -> Result<DigitalWalletProvider, LedgerError> {
    match s.to_lowercase().as_str() {
        "line-pay"  | "linepay"    => Ok(DigitalWalletProvider::LinePay),
        "apple-pay" | "applepay"   => Ok(DigitalWalletProvider::ApplePay),
        "google-pay"| "googlepay"  => Ok(DigitalWalletProvider::GooglePay),
        "jko-pay"   | "jkopay"     => Ok(DigitalWalletProvider::JkoPay),
        "pi-wallet" | "piwallet"   => Ok(DigitalWalletProvider::PiWallet),
        "taiwan-pay"| "taiwanpay"  => Ok(DigitalWalletProvider::TaiwanPay),
        other                      => Ok(DigitalWalletProvider::Other(other.to_string())),
    }
}

pub fn parse_investment_type(s: &str) -> Result<InvestmentType, LedgerError> {
    match s.to_lowercase().as_str() {
        "stock"                       => Ok(InvestmentType::Stock),
        "etf"                         => Ok(InvestmentType::Etf),
        "mutual-fund" | "mutualfund"  => Ok(InvestmentType::MutualFund),
        "bond"                        => Ok(InvestmentType::Bond),
        "crypto"                      => Ok(InvestmentType::Crypto),
        other                         => Ok(InvestmentType::Other(other.to_string())),
    }
}
