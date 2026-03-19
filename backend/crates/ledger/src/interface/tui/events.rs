//! Keyboard event → app state transitions.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::app::{App, Form, InputMode, Screen};

/// What the event loop should do after handling a key.
pub enum EventOutcome {
    /// Keep running, nothing special
    Continue,
    /// Reload accounts + net worth from the repo
    Reload,
    /// Submit the current form — returns collected field values
    SubmitForm(Vec<String>),
    /// Remove the currently selected account
    RemoveSelected,
    /// Quit the application
    Quit,
}

pub fn handle_key(app: &mut App, key: KeyEvent) -> EventOutcome {
    match app.input_mode {
        InputMode::Normal => handle_normal(app, key),
        InputMode::Editing => handle_editing(app, key),
    }
}

// ── Normal mode ───────────────────────────────────────────────────────────────

fn handle_normal(app: &mut App, key: KeyEvent) -> EventOutcome {
    match &app.screen.clone() {
        Screen::Dashboard => match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                app.screen = Screen::Quit;
                EventOutcome::Quit
            }
            KeyCode::Char('Q') => {
                app.screen = Screen::Quit;
                EventOutcome::Quit
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.select_next();
                EventOutcome::Continue
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.select_prev();
                EventOutcome::Continue
            }
            KeyCode::Char('w') => {
                app.screen = Screen::NetWorth;
                EventOutcome::Continue
            }
            KeyCode::Char('n') => {
                app.screen = Screen::AddAccountMenu;
                EventOutcome::Continue
            }
            KeyCode::Char('d') | KeyCode::Delete => {
                if !app.accounts.is_empty() {
                    app.screen = Screen::ConfirmRemove;
                }
                EventOutcome::Continue
            }
            KeyCode::Char('r') => EventOutcome::Reload,
            _ => EventOutcome::Continue,
        },

        Screen::ConfirmRemove => match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                app.screen = Screen::Dashboard;
                EventOutcome::RemoveSelected
            }
            KeyCode::Char('n') | KeyCode::Esc => {
                app.screen = Screen::Dashboard;
                EventOutcome::Continue
            }
            _ => EventOutcome::Continue,
        },

        Screen::NetWorth => match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('w') => {
                app.screen = Screen::Dashboard;
                EventOutcome::Continue
            }
            _ => EventOutcome::Continue,
        },

        Screen::AddAccountMenu => match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.screen = Screen::Dashboard;
                EventOutcome::Continue
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.account_type_next();
                EventOutcome::Continue
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app.account_type_prev();
                EventOutcome::Continue
            }
            KeyCode::Enter => {
                let idx = app.account_type_cursor;
                app.open_form_for_type(idx);
                EventOutcome::Continue
            }
            _ => EventOutcome::Continue,
        },

        Screen::StatusMessage { .. } => {
            // Any key dismisses the status and reloads
            app.screen = Screen::Dashboard;
            EventOutcome::Reload
        }

        Screen::Quit => EventOutcome::Quit,
    }
}

// ── Editing mode (form) ───────────────────────────────────────────────────────

fn handle_editing(app: &mut App, key: KeyEvent) -> EventOutcome {
    let Some(form) = app.form.as_mut() else {
        app.input_mode = InputMode::Normal;
        return EventOutcome::Continue;
    };

    match key.code {
        KeyCode::Esc => {
            app.form = None;
            app.input_mode = InputMode::Normal;
            EventOutcome::Continue
        }

        KeyCode::Tab => {
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                form.prev_field();
            } else {
                form.next_field();
            }
            EventOutcome::Continue
        }

        KeyCode::BackTab => {
            form.prev_field();
            EventOutcome::Continue
        }

        KeyCode::Enter => {
            // If not on last field, advance; if on last, submit
            let last = form.fields.len() - 1;
            if form.focused < last {
                form.next_field();
                EventOutcome::Continue
            } else {
                let values: Vec<String> = form.fields.iter()
                    .map(|f| f.value.trim().to_string())
                    .collect();
                app.form = None;
                app.input_mode = InputMode::Normal;
                EventOutcome::SubmitForm(values)
            }
        }

        KeyCode::Backspace => {
            form.current_field_mut().value.pop();
            EventOutcome::Continue
        }

        KeyCode::Char(c) => {
            form.current_field_mut().value.push(c);
            EventOutcome::Continue
        }

        _ => EventOutcome::Continue,
    }
}
