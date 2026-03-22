//! `cargo run --bin seed -- --db test.db`
//!
//! Populates a fresh SQLite database with a realistic test dataset.
//!
//! Start the server against it with:
//!   cargo run -- --db test.db
//!
//! Accounts created:
//!   • CTBC Checking            (cash, USD, $8 450)
//!   • Taipei Fubon Savings     (cash, TWD, NT$120 000)
//!   • Cash Wallet              (physical wallet, TWD, NT$2 500)
//!   • LINE Pay                 (digital wallet, TWD, NT$1 800)
//!   • Fubon Securities         (investment, USD, $15 000 + AAPL/VOO/BTC)
//!   • CTBC Visa Platinum *4242 (credit card, USD, $3 200 limit, $850 outstanding)
//!   • Student Loan             (loan, USD, $18 400)
//! + ~15 ledger entries

use std::sync::Arc;
use ledger::{
    application::{
        ports::{LedgerEntryRepository, LedgerUnitOfWork, StatementRepository, UserFinancesRepository},
        use_cases::{
            create_user_finances, open_cash_account, add_physical_wallet,
            add_digital_wallet, open_investment_account, add_credit_card,
            open_loan_account, deposit_funds, withdraw_funds, transfer_funds,
            charge_credit_card, add_holding, make_payment,
        },
    },
    domain::currency::Currency,
    infrastructure::{
        open_db, SqliteUserFinancesRepository, SqliteLedgerEntryRepository,
        SqliteStatementRepository, SqliteLedgerUnitOfWork,
    },
    interface::resolve_user,
};
use clap::Parser;

fn d(s: &str) -> rust_decimal::Decimal { s.parse().expect(s) }

#[derive(Parser, Debug)]
#[command(name = "seed", about = "Populate a test SQLite database with sample data")]
struct Args {
    #[arg(long, default_value = "test.db")]
    db: String,
    #[arg(long)]
    user: Option<uuid::Uuid>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let pool = match open_db(&args.db).await {
        Ok(p)  => p,
        Err(e) => { eprintln!("seed: cannot open {}: {e}", args.db); std::process::exit(1); }
    };

    let repo: Arc<dyn UserFinancesRepository> =
        Arc::new(SqliteUserFinancesRepository::new(pool.clone()));
    let _entry_repo: Arc<dyn LedgerEntryRepository> =
        Arc::new(SqliteLedgerEntryRepository::new(pool.clone()));
    let stmt_repo: Arc<dyn StatementRepository> =
        Arc::new(SqliteStatementRepository::new(pool.clone()));
    let uow: Arc<dyn LedgerUnitOfWork> =
        Arc::new(SqliteLedgerUnitOfWork::new(pool));

    let owner = resolve_user(args.user);
    let usd   = Currency::USD;
    let twd   = Currency::TWD;

    // ── Init ──────────────────────────────────────────────────────────────────
    match create_user_finances::execute(
        uow.as_ref(),
        create_user_finances::CreateUserFinancesCommand { owner_id: owner },
    ).await {
        Ok(_)  => println!("✓ Created user finances"),
        Err(e) => {
            eprintln!("User already initialised ({e}). Delete {db} first.", db = args.db);
            std::process::exit(0);
        }
    }

    // ── Cash accounts ─────────────────────────────────────────────────────────
    open_cash_account::execute(uow.as_ref(), open_cash_account::OpenCashAccountCommand {
        owner_id: owner, currency: usd,
        name:            "CTBC Checking".to_string(),
        account_number:  "012-345-678901".to_string(),
        bank:            "CTBC Bank".to_string(),
        initial_balance: d("8450.00"),
    }).await.unwrap();
    println!("✓ CTBC Checking (USD $8 450)");

    open_cash_account::execute(uow.as_ref(), open_cash_account::OpenCashAccountCommand {
        owner_id: owner, currency: twd,
        name:            "Taipei Fubon Savings".to_string(),
        account_number:  "809-001-123456".to_string(),
        bank:            "Taipei Fubon Bank".to_string(),
        initial_balance: d("120000"),
    }).await.unwrap();
    println!("✓ Taipei Fubon Savings (TWD NT$120 000)");

    // ── Wallets ───────────────────────────────────────────────────────────────
    add_physical_wallet::execute(uow.as_ref(), add_physical_wallet::AddPhysicalWalletCommand {
        owner_id: owner, currency: twd,
        name:            "Cash Wallet".to_string(),
        initial_balance: d("2500"),
    }).await.unwrap();
    println!("✓ Cash Wallet (TWD NT$2 500)");

    add_digital_wallet::execute(uow.as_ref(), add_digital_wallet::AddDigitalWalletCommand {
        owner_id: owner, currency: twd,
        name:                "LINE Pay".to_string(),
        provider:            "line-pay".to_string(),
        provider_account_id: "user@example.com".to_string(),
        initial_balance:     d("1800"),
    }).await.unwrap();
    println!("✓ LINE Pay (TWD NT$1 800)");

    // ── Investment ────────────────────────────────────────────────────────────
    open_investment_account::execute(uow.as_ref(), open_investment_account::OpenInvestmentAccountCommand {
        owner_id: owner, currency: usd,
        name:           "Fubon Securities".to_string(),
        account_number: "F-78901234".to_string(),
        bank:           "Fubon Securities".to_string(),
        cash_balance:   d("15000"),
    }).await.unwrap();
    println!("✓ Fubon Securities (USD $15 000 cash)");

    // Reload to get account IDs
    let finances = repo.load(owner).await.unwrap();
    let ctbc_id  = finances.accounts().iter()
        .find(|a| a.account_name_str() == "CTBC Checking").unwrap().account_id();
    let fubon_id = finances.accounts().iter()
        .find(|a| a.account_name_str() == "Taipei Fubon Savings").unwrap().account_id();
    let inv_id   = finances.accounts().iter()
        .find(|a| a.account_name_str() == "Fubon Securities").unwrap().account_id();

    for (ticker, inv_type, qty, price) in [
        ("AAPL",  "stock",  "10",   "189.50"),
        ("VOO",   "etf",    "5",    "498.75"),
        ("BTC",   "crypto", "0.25", "67800.00"),
    ] {
        add_holding::execute(uow.as_ref(), add_holding::AddHoldingCommand {
            owner_id: owner, currency: usd, account_id: inv_id,
            ticker:          ticker.to_string(),
            investment_type: inv_type.to_string(),
            quantity:        d(qty),
            unit_price:      d(price),
        }).await.unwrap();
        println!("  ✓ Holding {ticker}");
    }

    // ── Credit card ───────────────────────────────────────────────────────────
    add_credit_card::execute(uow.as_ref(), add_credit_card::AddCreditCardCommand {
        owner_id: owner, currency: usd,
        name:               "CTBC Visa Platinum".to_string(),
        last_four:          "4242".to_string(),
        network:            "visa".to_string(),
        expiry_month:       9,
        expiry_year:        2028,
        credit_limit:       d("3200"),
        outstanding:        d("850"),
        cash_advance_limit: None,
        statement_day:      25,
        due_day:            15,
        interest_rate:      Some(d("19.99")),
    }).await.unwrap();
    println!("✓ CTBC Visa Platinum *4242 (USD, $3 200 limit)");

    // ── Loan ──────────────────────────────────────────────────────────────────
    open_loan_account::execute(uow.as_ref(), open_loan_account::OpenLoanAccountCommand {
        owner_id: owner, currency: usd,
        name:            "Student Loan".to_string(),
        account_number:  None,
        bank:            "Ministry of Education".to_string(),
        creditor:        "Taiwan Student Loan Fund".to_string(),
        principal:       d("18400"),
        interest_rate:   Some(d("2.5")),
        due_day:         Some(20),
        maturity_date:   None,
        minimum_payment: Some(d("320")),
    }).await.unwrap();
    println!("✓ Student Loan (USD $18 400)");

    let finances = repo.load(owner).await.unwrap();
    let cc_id   = finances.accounts().iter()
        .find(|a| a.account_name_str() == "CTBC Visa Platinum").unwrap().account_id();
    let loan_id = finances.accounts().iter()
        .find(|a| a.account_name_str() == "Student Loan").unwrap().account_id();

    // ── Transactions ──────────────────────────────────────────────────────────
    deposit_funds::execute(uow.as_ref(), deposit_funds::DepositFundsCommand {
        owner_id: owner, account_id: ctbc_id, currency: usd,
        amount:      d("3200"),
        label:       Some("Salary".to_string()),
        description: Some("March paycheck".to_string()),
    }).await.unwrap();

    withdraw_funds::execute(uow.as_ref(), withdraw_funds::WithdrawFundsCommand {
        owner_id: owner, account_id: ctbc_id, currency: usd,
        amount:      d("1100"),
        label:       Some("Rent".to_string()),
        description: Some("March rent".to_string()),
    }).await.unwrap();

    withdraw_funds::execute(uow.as_ref(), withdraw_funds::WithdrawFundsCommand {
        owner_id: owner, account_id: ctbc_id, currency: usd,
        amount:      d("245.80"),
        label:       Some("Groceries".to_string()),
        description: None,
    }).await.unwrap();

    transfer_funds::execute(uow.as_ref(), transfer_funds::TransferFundsCommand {
        owner_id: owner, currency: usd,
        from_account_id: ctbc_id,
        to_account_id:   fubon_id,
        amount:          d("500"),
        label:           Some("Savings transfer".to_string()),
        description:     None,
    }).await.unwrap();
    println!("✓ Transfer CTBC → Fubon $500");

    for (amt, label) in [
        ("45.00",  "Lunch"),
        ("120.50", "Clothing"),
        ("18.99",  "Streaming subscription"),
        ("320.00", "Flight tickets"),
        ("89.75",  "Electronics"),
    ] {
        charge_credit_card::execute(uow.as_ref(), charge_credit_card::ChargeCreditCardCommand {
            owner_id: owner, account_id: cc_id, currency: usd,
            amount:      d(amt),
            label:       Some(label.to_string()),
            description: None,
        }).await.unwrap();
    }
    println!("✓ 5 credit card charges");

    make_payment::execute(uow.as_ref(), stmt_repo.as_ref(), make_payment::MakePaymentCommand {
        owner_id: owner, currency: usd,
        debt_account_id:  cc_id,
        from_account_id:  ctbc_id,
        amount:           d("400"),
        label:            Some("CC payment".to_string()),
        description:      None,
    }).await.unwrap();

    make_payment::execute(uow.as_ref(), stmt_repo.as_ref(), make_payment::MakePaymentCommand {
        owner_id: owner, currency: usd,
        debt_account_id:  loan_id,
        from_account_id:  ctbc_id,
        amount:           d("320"),
        label:            Some("Student loan payment".to_string()),
        description:      None,
    }).await.unwrap();
    println!("✓ CC payment $400, loan payment $320");

    deposit_funds::execute(uow.as_ref(), deposit_funds::DepositFundsCommand {
        owner_id: owner, account_id: fubon_id, currency: twd,
        amount:      d("15000"),
        label:       Some("Bonus".to_string()),
        description: Some("Q1 performance bonus".to_string()),
    }).await.unwrap();

    withdraw_funds::execute(uow.as_ref(), withdraw_funds::WithdrawFundsCommand {
        owner_id: owner, account_id: fubon_id, currency: twd,
        amount:      d("8500"),
        label:       Some("Utilities".to_string()),
        description: None,
    }).await.unwrap();

    println!();
    println!("✓ Seed complete → {db}", db = args.db);
    println!();
    println!("Start the server:");
    println!("  cargo run -- --db {db}", db = args.db);
}
