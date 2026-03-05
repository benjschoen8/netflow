#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardNetwork {
    Visa,
    Mastercard,
    AmericanExpress,
    UnionPay,
    Discover,
    Other(String),
}

impl fmt::Display for CardNetwork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            CardNetwork::Visa => "Visa",
            CardNetwork::Mastercard => "Mastercard",
            CardNetwork::AmericanExpress => "American Express",
            CardNetwork::UnionPay => "UnionPay",
            CardNetwork::Discover => "Discover",
            CardNetwork::Other(s) => s.as_str(),
        };
        write!(f, "{}", name)
    }
}