//! TUI event loop — wires the app state, renderer, and use cases together.

use std::time::Duration;
use std::str::FromStr;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use shared::domain::UserId;

use crate::application::{
    error::LedgerError,
    parse_helpers::{parse_card_network, parse_investment_type, parse_wallet_provider},
    ports::UserFinancesRepository,
    use_cases::{
        get_net_worth::{self, GetNetWorthQuery},
        list_accounts::{self, ListAccountsQuery},
        open_cash_account::{self, OpenCashAccountCommand},
        add_physical_wallet::{self, AddPhysicalWalletCommand},
        add_digital_wallet::{self, AddDigitalWalletCommand},
        open_investment_account::{self, OpenInvestmentAccountCommand},
        add_credit_card::{self, AddCreditCardCommand},
        open_loan_account::{self, OpenLoanAccountCommand},
        remove_account::{self, RemoveAccountCommand},
    },
};
use crate::domain::{
    account_id::AccountId,
    currency::Currency,
};

use super::{app::App, events::{handle_key, EventOutcome}, ui};

// ── Data loader ───────────────────────────────────────────────────────────────

async fn reload(
    app: &mut App,
    repo: &dyn UserFinancesRepository,
    user_id: UserId,
) {
    match list_accounts::execute(repo, ListAccountsQuery { owner_id: user_id }).await {
        Ok(accounts) => app.accounts = accounts,
        Err(LedgerError::FinancesNotFound) => app.accounts = vec![],
        Err(e) => app.set_status(e.to_string(), true),
    }

    match get_net_worth::execute(repo, GetNetWorthQuery { owner_id: user_id, currency: None }).await {
        Ok(nw) => app.net_worth = nw,
        Err(_) => app.net_worth = vec![],
    }
}

// ── Form submission ───────────────────────────────────────────────────────────

async fn submit_form(
    type_idx: usize,
    values: Vec<String>,
    repo: &dyn UserFinancesRepository,
    user_id: UserId,
) -> Result<(), LedgerError> {
    let v = |i: usize| values.get(i).map(String::as_str).unwrap_or("").to_string();

    match type_idx {
        // Cash account
        0 => open_cash_account::execute(repo, OpenCashAccountCommand {
            owner_id:        user_id,
            name:            v(0),
            account_number:  v(1),
            bank:            v(2),
            currency:        v(3).parse::<Currency>().map_err(LedgerError::Validation)?,
            initial_balance: v(4).parse().map_err(|_| LedgerError::Validation("Invalid balance".into()))?,
        }).await,

        // Physical wallet
        1 => add_physical_wallet::execute(repo, AddPhysicalWalletCommand {
            owner_id:        user_id,
            name:            v(0),
            currency:        v(1).parse::<Currency>().map_err(LedgerError::Validation)?,
            initial_balance: v(2).parse().map_err(|_| LedgerError::Validation("Invalid balance".into()))?,
        }).await,

        // Digital wallet
        2 => add_digital_wallet::execute(repo, AddDigitalWalletCommand {
            owner_id:            user_id,
            name:                v(0),
            provider:            v(1),
            provider_account_id: v(2),
            currency:            v(3).parse::<Currency>().map_err(LedgerError::Validation)?,
            initial_balance:     v(4).parse().map_err(|_| LedgerError::Validation("Invalid balance".into()))?,
        }).await,

        // Investment account
        3 => open_investment_account::execute(repo, OpenInvestmentAccountCommand {
            owner_id:       user_id,
            name:           v(0),
            account_number: v(1),
            bank:           v(2),
            currency:       v(3).parse::<Currency>().map_err(LedgerError::Validation)?,
            cash_balance:   v(4).parse().map_err(|_| LedgerError::Validation("Invalid balance".into()))?,
        }).await,

        // Credit card
        4 => add_credit_card::execute(repo, AddCreditCardCommand {
            owner_id:           user_id,
            name:               v(0),
            last_four:          v(1),
            network:            v(2),
            expiry_month:       v(3).parse().map_err(|_| LedgerError::Validation("Invalid month".into()))?,
            expiry_year:        v(4).parse().map_err(|_| LedgerError::Validation("Invalid year".into()))?,
            credit_limit:       v(5).parse().map_err(|_| LedgerError::Validation("Invalid limit".into()))?,
            currency:           v(6).parse::<Currency>().map_err(LedgerError::Validation)?,
            outstanding:        v(7).parse().map_err(|_| LedgerError::Validation("Invalid outstanding".into()))?,
            cash_advance_limit: None,
            statement_day:      v(8).parse().map_err(|_| LedgerError::Validation("Invalid statement day".into()))?,
            due_day:            v(9).parse().map_err(|_| LedgerError::Validation("Invalid due day".into()))?,
            interest_rate:      if v(10).is_empty() { None } else {
                Some(v(10).parse().map_err(|_| LedgerError::Validation("Invalid rate".into()))?)
            },
        }).await,

        // Loan account
        5 => open_loan_account::execute(repo, OpenLoanAccountCommand {
            owner_id:        user_id,
            name:            v(0),
            account_number:  None,
            bank:            v(1),
            creditor:        v(2),
            currency:        v(3).parse::<Currency>().map_err(LedgerError::Validation)?,
            principal:       v(4).parse().map_err(|_| LedgerError::Validation("Invalid principal".into()))?,
            interest_rate:   if v(5).is_empty() { None } else {
                Some(v(5).parse().map_err(|_| LedgerError::Validation("Invalid rate".into()))?)
            },
            due_day:         if v(6).is_empty() { None } else {
                Some(v(6).parse().map_err(|_| LedgerError::Validation("Invalid due day".into()))?)
            },
            maturity_date:   None,
            minimum_payment: None,
        }).await,

        _ => Ok(()),
    }
}

// ── Main TUI entry point ──────────────────────────────────────────────────────

pub async fn run(
    repo: &dyn UserFinancesRepository,
    user_id: UserId,
) -> Result<(), Box<dyn std::error::Error>> {
    // ── Terminal setup ────────────────────────────────────────────────────────
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    // Remember which account type was selected before opening the form
    let mut pending_type_idx: usize = 0;

    // Initial data load
    reload(&mut app, repo, user_id).await;

    // ── Event loop ────────────────────────────────────────────────────────────
    loop {
        terminal.draw(|frame| ui::render(frame, &app))?;

        if !event::poll(Duration::from_millis(250))? {
            continue;
        }

        let Event::Key(key) = event::read()? else { continue };

        // Capture account_type_cursor BEFORE handle_key might change it
        // (needed so SubmitForm knows which form was submitted)
        let type_idx_before = app.account_type_cursor;

        match handle_key(&mut app, key) {
            EventOutcome::Continue => {}

            EventOutcome::Reload => {
                reload(&mut app, repo, user_id).await;
            }

            EventOutcome::SubmitForm(values) => {
                // pending_type_idx was set when we called open_form_for_type
                match submit_form(pending_type_idx, values, repo, user_id).await {
                    Ok(()) => {
                        app.set_status("Account added successfully!".into(), false);
                        reload(&mut app, repo, user_id).await;
                    }
                    Err(e) => {
                        app.set_status(e.to_string(), true);
                    }
                }
            }

            EventOutcome::RemoveSelected => {
                if let Some(account) = app.accounts.get(app.selected) {
                    let id_str = account.account_id.clone();
                    match uuid::Uuid::parse_str(&id_str)
                        .map_err(|_| LedgerError::Validation("Bad UUID".into()))
                        .and_then(|u| AccountId::restore(u).map_err(LedgerError::Domain))
                    {
                        Ok(account_id) => {
                            match remove_account::execute(repo, remove_account::RemoveAccountCommand {
                                owner_id: user_id,
                                account_id,
                            }).await {
                                Ok(()) => {
                                    app.selected = app.selected.saturating_sub(1);
                                    reload(&mut app, repo, user_id).await;
                                }
                                Err(e) => app.set_status(e.to_string(), true),
                            }
                        }
                        Err(e) => app.set_status(e.to_string(), true),
                    }
                }
            }

            EventOutcome::Quit => break,
        }

        // Track which type was selected when the form opens
        if app.form.is_some() {
            pending_type_idx = type_idx_before;
        }
    }

    // ── Cleanup ───────────────────────────────────────────────────────────────
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
