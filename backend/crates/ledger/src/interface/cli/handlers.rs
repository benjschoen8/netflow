//! Dispatch layer: translate parsed CLI args into application use-case calls
//! and print results to stdout.

use shared::domain::UserId;
use uuid::Uuid;

use crate::application::error::LedgerError;
use crate::application::ports::UserFinancesRepository;
use crate::application::use_cases::{
    accrue_interest, add_credit_card, add_digital_wallet, add_holding,
    add_physical_wallet, charge_credit_card, close_statement,
    create_user_finances, deposit_funds, get_net_worth,
    list_accounts, make_payment, open_cash_account, open_investment_account,
    open_loan_account, remove_account, remove_holding, revoke_temporary_limit,
    update_holding_price, withdraw_funds,
};
use crate::domain::account_id::AccountId;

use super::commands::{AccountCommands, Commands, HoldingCommands};
use super::parse_helpers::parse_naive_date;

// ── User resolution ───────────────────────────────────────────────────────────

/// Resolve the CLI `--user` flag (or env var) to a `UserId`.
/// In single-user mode a stable deterministic UUID is used so the user
/// never has to think about it.
pub fn resolve_user(user: Option<Uuid>) -> UserId {
    const DEFAULT_USER: &str = "00000000-0000-0000-0000-000000000001";
    let uuid = user.unwrap_or_else(|| DEFAULT_USER.parse().unwrap());
    UserId::restore(uuid).expect("valid user UUID")
}

// ── Main dispatch ─────────────────────────────────────────────────────────────

pub async fn dispatch(
    repo: &dyn UserFinancesRepository,
    user_id: UserId,
    command: Commands,
) -> Result<(), LedgerError> {
    match command {
        // ── Init ──────────────────────────────────────────────────────────────
        Commands::Init => {
            create_user_finances::execute(
                repo,
                create_user_finances::CreateUserFinancesCommand { owner_id: user_id },
            )
            .await?;
            println!("✓ Finances initialised. Ready to track your cash flow.");
        }

        // ── List ──────────────────────────────────────────────────────────────
        Commands::List => {
            let accounts = list_accounts::execute(
                repo,
                list_accounts::ListAccountsQuery { owner_id: user_id },
            )
            .await?;

            if accounts.is_empty() {
                println!("No accounts found. Use `netflow account` to add one.");
                return Ok(());
            }

            println!(
                "\n{:<36}  {:<22}  {:<14}  {:<4}  {:>14}",
                "ID", "Name", "Type", "CCY", "Balance"
            );
            println!("{}", "─".repeat(96));

            for a in accounts {
                let balance_str = if a.is_debt {
                    format!("{:>13}-", a.balance)
                } else {
                    format!("{:>14}", a.balance)
                };
                let overdue = if a.is_overdue { "  ⚠ OVERDUE" } else { "" };
                println!(
                    "{:<36}  {:<22}  {:<14}  {:<4}  {}{}",
                    a.account_id,
                    a.account_name,
                    a.account_type,
                    a.currency,
                    balance_str,
                    overdue
                );
            }
            println!();
        }

        // ── Net worth ─────────────────────────────────────────────────────────
        Commands::NetWorth { currency } => {
            let results = get_net_worth::execute(
                repo,
                get_net_worth::GetNetWorthQuery {
                    owner_id: user_id,
                    currency: currency.map(Into::into),
                },
            )
            .await?;

            let separator = "═".repeat(40);
            println!();
            for r in &results {
                println!("{}", separator);
                println!("Currency  : {}", r.currency);
                println!("Assets    : {:>16}", r.total_assets);
                println!("Debts     : {:>16}", r.total_debts);
                let label = if r.is_deficit { "deficit" } else { "surplus" };
                println!("Net worth : {:>16}  ({})", r.net_worth, label);
            }
            println!("{}", separator);
            println!();
        }

        // ── Deposit ───────────────────────────────────────────────────────────
        Commands::Deposit { account, amount, currency } => {
            deposit_funds::execute(
                repo,
                deposit_funds::DepositFundsCommand {
                    owner_id:   user_id,
                    account_id: AccountId::restore(account)?,
                    amount,
                    currency:   currency.into(),
                },
            )
            .await?;
            println!("✓ Deposited {} to account {}.", amount, account);
        }

        // ── Withdraw ──────────────────────────────────────────────────────────
        Commands::Withdraw { account, amount, currency } => {
            withdraw_funds::execute(
                repo,
                withdraw_funds::WithdrawFundsCommand {
                    owner_id:   user_id,
                    account_id: AccountId::restore(account)?,
                    amount,
                    currency:   currency.into(),
                },
            )
            .await?;
            println!("✓ Withdrew {} from account {}.", amount, account);
        }

        // ── Pay ───────────────────────────────────────────────────────────────
        Commands::Pay { from, to, amount, currency } => {
            make_payment::execute(
                repo,
                make_payment::MakePaymentCommand {
                    owner_id:        user_id,
                    from_account_id: AccountId::restore(from)?,
                    debt_account_id: AccountId::restore(to)?,
                    amount,
                    currency:        currency.into(),
                },
            )
            .await?;
            println!("✓ Payment of {} recorded: {} → {}.", amount, from, to);
        }

        // ── Charge ────────────────────────────────────────────────────────────
        Commands::Charge { account, amount, currency } => {
            charge_credit_card::execute(
                repo,
                charge_credit_card::ChargeCreditCardCommand {
                    owner_id:   user_id,
                    account_id: AccountId::restore(account)?,
                    amount,
                    currency:   currency.into(),
                },
            )
            .await?;
            println!("✓ Charged {} to card {}.", amount, account);
        }


        // ── Grant temporary limit ─────────────────────────────────────────────
        Commands::GrantLimit { account, limit, currency, expires } => {
            use crate::application::use_cases::grant_temporary_limit;
            let expires_on = super::parse_helpers::parse_naive_date(&expires)?;
            grant_temporary_limit::execute(
                repo,
                grant_temporary_limit::GrantTemporaryLimitCommand {
                    owner_id:   user_id,
                    account_id: AccountId::restore(account)?,
                    new_limit:  limit,
                    currency:   currency.into(),
                    expires_on,
                },
            )
            .await?;
            println!("✓ Temporary limit granted for card {}.", account);
        }

        // ── Close statement ───────────────────────────────────────────────────
        Commands::CloseStatement { account, min_payment, currency } => {
            close_statement::execute(
                repo,
                close_statement::CloseStatementCommand {
                    owner_id:        user_id,
                    account_id:      AccountId::restore(account)?,
                    minimum_payment: min_payment,
                    currency:        currency.into(),
                },
            )
            .await?;
            println!("✓ Statement closed for card {}.", account);
        }

        // ── Revoke temporary limit ────────────────────────────────────────────
        Commands::RevokeLimit { account } => {
            revoke_temporary_limit::execute(
                repo,
                revoke_temporary_limit::RevokeTemporaryLimitCommand {
                    owner_id:   user_id,
                    account_id: AccountId::restore(account)?,
                },
            )
            .await?;
            println!("✓ Temporary credit limit revoked for card {}.", account);
        }

        // ── Accrue interest ───────────────────────────────────────────────────
        Commands::AccrueInterest { account } => {
            accrue_interest::execute(
                repo,
                accrue_interest::AccrueInterestCommand {
                    owner_id:   user_id,
                    account_id: AccountId::restore(account)?,
                },
            )
            .await?;
            println!("✓ Interest accrued for account {}.", account);
        }

        // ── Account subcommands ───────────────────────────────────────────────
        Commands::Account(sub) => handle_account(repo, user_id, sub).await?,

        // ── Holding subcommands ───────────────────────────────────────────────
        Commands::Holding(sub) => handle_holding(repo, user_id, sub).await?,
    }

    Ok(())
}

// ── Account subcommand handler ────────────────────────────────────────────────

async fn handle_account(
    repo: &dyn UserFinancesRepository,
    user_id: UserId,
    cmd: AccountCommands,
) -> Result<(), LedgerError> {
    match cmd {
        AccountCommands::Cash { name, number, bank, currency, balance } => {
            open_cash_account::execute(
                repo,
                open_cash_account::OpenCashAccountCommand {
                    owner_id:        user_id,
                    name,
                    account_number:  number,
                    bank,
                    currency:        currency.into(),
                    initial_balance: balance,
                },
            )
            .await?;
            println!("✓ Cash account opened.");
        }

        AccountCommands::Wallet { name, currency, balance } => {
            add_physical_wallet::execute(
                repo,
                add_physical_wallet::AddPhysicalWalletCommand {
                    owner_id:        user_id,
                    name,
                    currency:        currency.into(),
                    initial_balance: balance,
                },
            )
            .await?;
            println!("✓ Physical wallet added.");
        }

        AccountCommands::DigitalWallet { name, provider, provider_id, currency, balance } => {
            add_digital_wallet::execute(
                repo,
                add_digital_wallet::AddDigitalWalletCommand {
                    owner_id:            user_id,
                    name,
                    provider,
                    provider_account_id: provider_id,
                    currency:            currency.into(),
                    initial_balance:     balance,
                },
            )
            .await?;
            println!("✓ Digital wallet added.");
        }

        AccountCommands::Investment { name, number, bank, currency, cash } => {
            open_investment_account::execute(
                repo,
                open_investment_account::OpenInvestmentAccountCommand {
                    owner_id:       user_id,
                    name,
                    account_number: number,
                    bank,
                    currency:       currency.into(),
                    cash_balance:   cash,
                },
            )
            .await?;
            println!("✓ Investment account opened.");
        }

        AccountCommands::CreditCard {
            name, last_four, network, expiry_month, expiry_year,
            limit, currency, outstanding, statement_day, due_day, rate,
        } => {
            add_credit_card::execute(
                repo,
                add_credit_card::AddCreditCardCommand {
                    owner_id:           user_id,
                    name,
                    last_four,
                    network,
                    expiry_month,
                    expiry_year,
                    credit_limit:       limit,
                    currency:           currency.into(),
                    outstanding,
                    cash_advance_limit: None,
                    statement_day,
                    due_day,
                    interest_rate:      rate,
                },
            )
            .await?;
            println!("✓ Credit card added.");
        }

        AccountCommands::Loan {
            name, bank, creditor, currency, principal,
            number, rate, due_day, maturity, min_payment,
        } => {
            let maturity_date = maturity.as_deref().map(parse_naive_date).transpose()?;
            open_loan_account::execute(
                repo,
                open_loan_account::OpenLoanAccountCommand {
                    owner_id:        user_id,
                    name,
                    account_number:  number,
                    bank,
                    creditor,
                    currency:        currency.into(),
                    principal,
                    interest_rate:   rate,
                    due_day,
                    maturity_date,
                    minimum_payment: min_payment,
                },
            )
            .await?;
            println!("✓ Loan account opened.");
        }

        AccountCommands::Remove { id } => {
            remove_account::execute(
                repo,
                remove_account::RemoveAccountCommand {
                    owner_id:   user_id,
                    account_id: AccountId::restore(id)?,
                },
            )
            .await?;
            println!("✓ Account {} removed.", id);
        }
    }
    Ok(())
}

// ── Holding subcommand handler ────────────────────────────────────────────────

async fn handle_holding(
    repo: &dyn UserFinancesRepository,
    user_id: UserId,
    cmd: HoldingCommands,
) -> Result<(), LedgerError> {
    match cmd {
        HoldingCommands::Add { account, ticker, kind, qty, price, currency } => {
            add_holding::execute(
                repo,
                add_holding::AddHoldingCommand {
                    owner_id:        user_id,
                    account_id:      AccountId::restore(account)?,
                    ticker,
                    investment_type: kind,
                    quantity:        qty,
                    unit_price:      price,
                    currency:        currency.into(),
                },
            )
            .await?;
            println!("✓ Holding added.");
        }

        HoldingCommands::Remove { account, ticker } => {
            remove_holding::execute(
                repo,
                remove_holding::RemoveHoldingCommand {
                    owner_id:   user_id,
                    account_id: AccountId::restore(account)?,
                    ticker,
                },
            )
            .await?;
            println!("✓ Holding removed.");
        }

        HoldingCommands::UpdatePrice { account, ticker, price, currency } => {
            update_holding_price::execute(
                repo,
                update_holding_price::UpdateHoldingPriceCommand {
                    owner_id:   user_id,
                    account_id: AccountId::restore(account)?,
                    ticker,
                    new_price:  price,
                    currency:   currency.into(),
                },
            )
            .await?;
            println!("✓ Holding price updated.");
        }
    }
    Ok(())
}
