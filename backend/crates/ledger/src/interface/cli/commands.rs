//! All CLI subcommands, defined with `clap` derive macros.

use clap::{Parser, Subcommand, ValueEnum};
use rust_decimal::Decimal;
use uuid::Uuid;

// ── Root ──────────────────────────────────────────────────────────────────────

/// netflow — a terminal ledger for tracking your cash flow.
#[derive(Parser, Debug)]
#[command(name = "netflow", version, about, long_about = None)]
pub struct Cli {
    /// Path to the SQLite database file.
    #[arg(long, global = true, default_value = "netflow.db", env = "NETFLOW_DB")]
    pub db: String,

    /// The user ID (UUID). Defaults to the value in NETFLOW_USER env var,
    /// or a well-known default UUID for single-user mode.
    #[arg(long, global = true, env = "NETFLOW_USER")]
    pub user: Option<Uuid>,

    /// Omit to open the interactive TUI.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

// ── Top-level commands ────────────────────────────────────────────────────────

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialise finances for the current user (run once).
    Init,

    /// Manage accounts (add, remove, list).
    #[command(subcommand)]
    Account(AccountCommands),

    /// Record a deposit into an asset account.
    Deposit {
        /// Account ID (UUID).
        #[arg(short, long)]
        account: Uuid,
        /// Amount (e.g. 1500.00).
        #[arg(short = 'n', long)]
        amount: Decimal,
        /// Currency: usd or twd.
        #[arg(short, long, default_value = "twd")]
        currency: CurrencyArg,
    },

    /// Record a withdrawal from an asset account.
    Withdraw {
        #[arg(short, long)]
        account: Uuid,
        #[arg(short = 'n', long)]
        amount: Decimal,
        #[arg(short, long, default_value = "twd")]
        currency: CurrencyArg,
    },

    /// Pay down a loan or credit card from an asset account.
    Pay {
        /// Source asset account ID.
        #[arg(short, long)]
        from: Uuid,
        /// Debt account ID (loan or credit card).
        #[arg(short, long)]
        to: Uuid,
        #[arg(short = 'n', long)]
        amount: Decimal,
        #[arg(short, long, default_value = "twd")]
        currency: CurrencyArg,
    },

    /// Charge a purchase to a credit card.
    Charge {
        #[arg(short, long)]
        account: Uuid,
        #[arg(short = 'n', long)]
        amount: Decimal,
        #[arg(short, long, default_value = "twd")]
        currency: CurrencyArg,
    },

    /// Manage investment holdings.
    #[command(subcommand)]
    Holding(HoldingCommands),

    /// Show net worth summary.
    NetWorth {
        /// Filter to a specific currency. Shows all currencies if omitted.
        #[arg(short, long)]
        currency: Option<CurrencyArg>,
    },


    /// Grant a temporary credit limit increase to a credit card.
    GrantLimit {
        #[arg(short, long)]
        account: Uuid,
        /// New (higher) limit amount.
        #[arg(short = 'n', long)]
        limit: Decimal,
        #[arg(short, long, default_value = "twd")]
        currency: CurrencyArg,
        /// Expiry date for the temporary limit (YYYY-MM-DD).
        #[arg(short, long)]
        expires: String,
    },

    /// Close the billing statement on a credit card (sets statement balance).
    CloseStatement {
        #[arg(short, long)]
        account: Uuid,
        /// Optional minimum payment amount.
        #[arg(short = 'm', long)]
        min_payment: Option<Decimal>,
        #[arg(short, long, default_value = "twd")]
        currency: CurrencyArg,
    },

    /// Revoke a temporary credit limit granted to a credit card.
    RevokeLimit {
        #[arg(short, long)]
        account: Uuid,
    },

    /// Apply one month of interest to a debt account (loan or credit card).
    AccrueInterest {
        #[arg(short, long)]
        account: Uuid,
    },

    /// List all accounts and their balances.
    List,
}

// ── Account subcommands ───────────────────────────────────────────────────────

#[derive(Subcommand, Debug)]
pub enum AccountCommands {
    /// Open a bank cash account.
    Cash {
        #[arg(short, long)]
        name: String,
        #[arg(long)]
        number: String,
        #[arg(long)]
        bank: String,
        #[arg(short, long, default_value = "twd")]
        currency: CurrencyArg,
        #[arg(long, default_value = "0")]
        balance: Decimal,
    },

    /// Add a physical (cash-in-wallet) account.
    Wallet {
        #[arg(short, long)]
        name: String,
        #[arg(short, long, default_value = "twd")]
        currency: CurrencyArg,
        #[arg(long, default_value = "0")]
        balance: Decimal,
    },

    /// Add a digital wallet (LINE Pay, Apple Pay, etc.).
    DigitalWallet {
        #[arg(short, long)]
        name: String,
        /// Provider: line-pay, apple-pay, google-pay, jko-pay, pi-wallet,
        /// taiwan-pay, or other:<name>.
        #[arg(short, long)]
        provider: String,
        /// The account ID / phone number within the provider.
        #[arg(long)]
        provider_id: String,
        #[arg(short, long, default_value = "twd")]
        currency: CurrencyArg,
        #[arg(long, default_value = "0")]
        balance: Decimal,
    },

    /// Open a brokerage / investment account.
    Investment {
        #[arg(short, long)]
        name: String,
        #[arg(long)]
        number: String,
        #[arg(long)]
        bank: String,
        #[arg(short, long, default_value = "twd")]
        currency: CurrencyArg,
        #[arg(long, default_value = "0")]
        cash: Decimal,
    },

    /// Add a credit card.
    CreditCard {
        #[arg(short, long)]
        name: String,
        /// Last 4 digits (e.g. 4321).
        #[arg(long)]
        last_four: String,
        /// Network: visa, mastercard, amex, unionpay, discover, other:<name>.
        #[arg(long)]
        network: String,
        /// Expiry month (1-12).
        #[arg(long)]
        expiry_month: u8,
        /// Expiry year (e.g. 2028).
        #[arg(long)]
        expiry_year: u16,
        /// Credit limit amount.
        #[arg(long)]
        limit: Decimal,
        #[arg(short, long, default_value = "twd")]
        currency: CurrencyArg,
        /// Current outstanding balance (default: 0).
        #[arg(long, default_value = "0")]
        outstanding: Decimal,
        /// Day of month the statement closes.
        #[arg(long, default_value = "15")]
        statement_day: u8,
        /// Day of month payment is due.
        #[arg(long, default_value = "10")]
        due_day: u8,
        /// Annual interest rate % (e.g. 15.99).
        #[arg(long)]
        rate: Option<Decimal>,
    },

    /// Open a loan account.
    Loan {
        #[arg(short, long)]
        name: String,
        #[arg(long)]
        bank: String,
        #[arg(long)]
        creditor: String,
        #[arg(short, long, default_value = "twd")]
        currency: CurrencyArg,
        #[arg(long)]
        principal: Decimal,
        /// Optional account number.
        #[arg(long)]
        number: Option<String>,
        /// Annual interest rate %.
        #[arg(long)]
        rate: Option<Decimal>,
        /// Day of month payment is due (1-31).
        #[arg(long)]
        due_day: Option<u8>,
        /// Loan maturity date (YYYY-MM-DD).
        #[arg(long)]
        maturity: Option<String>,
        /// Fixed monthly minimum payment.
        #[arg(long)]
        min_payment: Option<Decimal>,
    },

    /// Remove an account by ID.
    Remove {
        #[arg(short, long)]
        id: Uuid,
    },
}

// ── Holding subcommands ───────────────────────────────────────────────────────

#[derive(Subcommand, Debug)]
pub enum HoldingCommands {
    /// Add or increase a holding in an investment account.
    Add {
        /// Investment account ID.
        #[arg(short, long)]
        account: Uuid,
        /// Ticker symbol (e.g. AAPL, BTC).
        #[arg(short, long)]
        ticker: String,
        /// Type: stock, etf, mutual-fund, bond, crypto, other:<name>.
        #[arg(long, default_value = "stock")]
        kind: String,
        #[arg(short, long)]
        qty: Decimal,
        #[arg(short, long)]
        price: Decimal,
        #[arg(short, long, default_value = "twd")]
        currency: CurrencyArg,
    },

    /// Remove a holding from an investment account entirely.
    Remove {
        #[arg(short, long)]
        account: Uuid,
        #[arg(short, long)]
        ticker: String,
    },

    /// Update the unit price of a holding.
    UpdatePrice {
        #[arg(short, long)]
        account: Uuid,
        #[arg(short, long)]
        ticker: String,
        #[arg(short, long)]
        price: Decimal,
        #[arg(short, long, default_value = "twd")]
        currency: CurrencyArg,
    },
}

// ── Shared arg types ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, ValueEnum)]
pub enum CurrencyArg {
    Usd,
    Twd,
}

impl From<CurrencyArg> for crate::domain::currency::Currency {
    fn from(c: CurrencyArg) -> Self {
        match c {
            CurrencyArg::Usd => Self::USD,
            CurrencyArg::Twd => Self::TWD,
        }
    }
}
