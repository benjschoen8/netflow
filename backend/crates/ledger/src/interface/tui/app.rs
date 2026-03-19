//! Central TUI state machine.
//! Holds all data the UI needs and drives screen transitions.

use crate::application::use_cases::list_accounts::AccountSummary;
use crate::application::use_cases::get_net_worth::NetWorthResult;

// ── Screens ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    /// Main account list
    Dashboard,
    /// Net worth breakdown
    NetWorth,
    /// Add account — step 1: pick type
    AddAccountMenu,
    /// Confirm removal of selected account
    ConfirmRemove,
    /// Show a transient status message then return to Dashboard
    StatusMessage { message: String, is_error: bool },
    /// Quit
    Quit,
}

// ── Input mode ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    /// Normal navigation — arrow keys, enter, etc.
    Normal,
    /// Typing into a form field
    Editing,
}

// ── Form state ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct FormField {
    pub label:       &'static str,
    pub value:       String,
    pub placeholder: &'static str,
}

impl FormField {
    pub fn new(label: &'static str, placeholder: &'static str) -> Self {
        Self { label, value: String::new(), placeholder }
    }
}

#[derive(Debug, Clone)]
pub struct Form {
    pub title:        &'static str,
    pub fields:       Vec<FormField>,
    pub focused:      usize,
    pub error:        Option<String>,
}

impl Form {
    pub fn current_field_mut(&mut self) -> &mut FormField {
        &mut self.fields[self.focused]
    }
    pub fn next_field(&mut self) {
        if self.focused + 1 < self.fields.len() {
            self.focused += 1;
        }
    }
    pub fn prev_field(&mut self) {
        if self.focused > 0 {
            self.focused -= 1;
        }
    }
    pub fn value(&self, idx: usize) -> &str {
        &self.fields[idx].value
    }
}

// ── App state ─────────────────────────────────────────────────────────────────

pub struct App {
    pub screen:       Screen,
    pub input_mode:   InputMode,

    // Dashboard data
    pub accounts:     Vec<AccountSummary>,
    pub selected:     usize,
    pub net_worth:    Vec<NetWorthResult>,

    // Active form (when adding an account)
    pub form:         Option<Form>,
    pub account_type_cursor: usize,
}

pub const ACCOUNT_TYPES: &[&str] = &[
    "Cash Account",
    "Physical Wallet",
    "Digital Wallet",
    "Investment Account",
    "Credit Card",
    "Loan Account",
];

impl App {
    pub fn new() -> Self {
        Self {
            screen:              Screen::Dashboard,
            input_mode:          InputMode::Normal,
            accounts:            Vec::new(),
            selected:            0,
            net_worth:           Vec::new(),
            form:                None,
            account_type_cursor: 0,
        }
    }

    // ── Navigation ────────────────────────────────────────────────────────────

    pub fn select_next(&mut self) {
        let len = self.accounts.len();
        if len > 0 {
            self.selected = (self.selected + 1) % len;
        }
    }

    pub fn select_prev(&mut self) {
        let len = self.accounts.len();
        if len > 0 {
            self.selected = self.selected.saturating_sub(1);
        }
    }

    pub fn selected_account(&self) -> Option<&AccountSummary> {
        self.accounts.get(self.selected)
    }

    pub fn account_type_next(&mut self) {
        self.account_type_cursor = (self.account_type_cursor + 1) % ACCOUNT_TYPES.len();
    }

    pub fn account_type_prev(&mut self) {
        if self.account_type_cursor > 0 {
            self.account_type_cursor -= 1;
        } else {
            self.account_type_cursor = ACCOUNT_TYPES.len() - 1;
        }
    }

    // ── Form builders ─────────────────────────────────────────────────────────

    pub fn open_form_for_type(&mut self, type_idx: usize) {
        let form = match type_idx {
            0 => Form {
                title: "Open Cash Account",
                focused: 0,
                error: None,
                fields: vec![
                    FormField::new("Name",    "e.g. CTBC Checking"),
                    FormField::new("Number",  "e.g. 12345678"),
                    FormField::new("Bank",    "e.g. CTBC"),
                    FormField::new("Currency","usd or twd"),
                    FormField::new("Balance", "0.00"),
                ],
            },
            1 => Form {
                title: "Add Physical Wallet",
                focused: 0,
                error: None,
                fields: vec![
                    FormField::new("Name",    "e.g. My Wallet"),
                    FormField::new("Currency","usd or twd"),
                    FormField::new("Balance", "0.00"),
                ],
            },
            2 => Form {
                title: "Add Digital Wallet",
                focused: 0,
                error: None,
                fields: vec![
                    FormField::new("Name",        "e.g. LINE Pay"),
                    FormField::new("Provider",    "line-pay / apple-pay / jko-pay..."),
                    FormField::new("Provider ID", "phone or account id"),
                    FormField::new("Currency",    "usd or twd"),
                    FormField::new("Balance",     "0.00"),
                ],
            },
            3 => Form {
                title: "Open Investment Account",
                focused: 0,
                error: None,
                fields: vec![
                    FormField::new("Name",     "e.g. Fubon Securities"),
                    FormField::new("Number",   "e.g. 88776655"),
                    FormField::new("Bank",     "e.g. Fubon"),
                    FormField::new("Currency", "usd or twd"),
                    FormField::new("Cash",     "0.00"),
                ],
            },
            4 => Form {
                title: "Add Credit Card",
                focused: 0,
                error: None,
                fields: vec![
                    FormField::new("Name",          "e.g. Citi Cashback"),
                    FormField::new("Last 4 digits", "4321"),
                    FormField::new("Network",       "visa / mastercard / amex..."),
                    FormField::new("Expiry month",  "1-12"),
                    FormField::new("Expiry year",   "e.g. 2028"),
                    FormField::new("Credit limit",  "e.g. 300000"),
                    FormField::new("Currency",      "usd or twd"),
                    FormField::new("Outstanding",   "0.00"),
                    FormField::new("Statement day", "15"),
                    FormField::new("Due day",       "10"),
                    FormField::new("Interest rate", "e.g. 15.99 (optional)"),
                ],
            },
            5 => Form {
                title: "Open Loan Account",
                focused: 0,
                error: None,
                fields: vec![
                    FormField::new("Name",       "e.g. Home Loan"),
                    FormField::new("Bank",       "e.g. Taipei Fubon"),
                    FormField::new("Creditor",   "e.g. Taipei Fubon Bank"),
                    FormField::new("Currency",   "usd or twd"),
                    FormField::new("Principal",  "e.g. 5000000"),
                    FormField::new("Rate %",     "e.g. 2.5 (optional)"),
                    FormField::new("Due day",    "e.g. 5 (optional)"),
                ],
            },
            _ => return,
        };
        self.form = Some(form);
        self.screen = Screen::Dashboard; // form overlays dashboard
        self.input_mode = InputMode::Editing;
    }

    pub fn set_status(&mut self, message: String, is_error: bool) {
        self.screen = Screen::StatusMessage { message, is_error };
    }
}
