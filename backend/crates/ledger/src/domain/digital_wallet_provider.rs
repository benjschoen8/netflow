use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigitalWalletProvider {
    LinePay,
    ApplePay,
    GooglePay,
    JkoPay,
    PiWallet,
    TaiwanPay,
    Other(String),
}

impl fmt::Display for DigitalWalletProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            DigitalWalletProvider::LinePay    => "LINE Pay",
            DigitalWalletProvider::ApplePay   => "Apple Pay",
            DigitalWalletProvider::GooglePay  => "Google Pay",
            DigitalWalletProvider::JkoPay     => "JKO Pay",
            DigitalWalletProvider::PiWallet   => "Pi Wallet",
            DigitalWalletProvider::TaiwanPay  => "Taiwan Pay",
            DigitalWalletProvider::Other(s)   => s.as_str(),
        };
        write!(f, "{}", name)
    }
}
