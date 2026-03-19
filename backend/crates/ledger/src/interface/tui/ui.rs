//! Ratatui rendering — pure functions, no state mutation.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, BorderType, Cell, Clear, List, ListItem, ListState,
        Padding, Paragraph, Row, Table, TableState, Wrap,
    },
};

use super::app::{App, Screen, InputMode, ACCOUNT_TYPES};

// ── Colour palette ────────────────────────────────────────────────────────────

const C_BG:        Color = Color::Rgb(18,  18,  28);   // dark navy
const C_SURFACE:   Color = Color::Rgb(28,  28,  42);   // slightly lighter
const C_BORDER:    Color = Color::Rgb(60,  60,  90);   // muted purple border
const C_ACCENT:    Color = Color::Rgb(120, 100, 240);  // purple accent
const C_GREEN:     Color = Color::Rgb(80,  200, 120);  // asset green
const C_RED:       Color = Color::Rgb(240,  80,  80);  // debt red
const C_YELLOW:    Color = Color::Rgb(240, 200,  60);  // warning yellow
const C_TEXT:      Color = Color::Rgb(200, 200, 220);  // main text
const C_DIM:       Color = Color::Rgb(100, 100, 130);  // dimmed text
const C_HIGHLIGHT: Color = Color::Rgb(50,  45,  90);   // selected row bg

// ── Entry point ───────────────────────────────────────────────────────────────

pub fn render(frame: &mut Frame, app: &App) {
    // Always clear with our background colour
    let bg = Block::default().style(Style::default().bg(C_BG));
    frame.render_widget(bg, frame.area());

    match &app.screen {
        Screen::Dashboard | Screen::ConfirmRemove => {
            render_dashboard(frame, app);
            if app.form.is_some() {
                render_form_overlay(frame, app);
            }
            if app.screen == Screen::ConfirmRemove {
                render_confirm_dialog(frame, app);
            }
        }
        Screen::NetWorth => render_net_worth(frame, app),
        Screen::AddAccountMenu => render_add_menu(frame, app),
        Screen::StatusMessage { message, is_error } => {
            render_dashboard(frame, app);
            render_status_overlay(frame, message, *is_error);
        }
        Screen::Quit => {}
    }
}

// ── Dashboard ─────────────────────────────────────────────────────────────────

fn render_dashboard(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // header
            Constraint::Min(0),      // table
            Constraint::Length(3),   // footer
        ])
        .split(area);

    render_header(frame, chunks[0], "  netflow — accounts");
    render_accounts_table(frame, chunks[1], app);
    render_footer(frame, chunks[2], &[
        ("↑↓", "select"), ("n", "new"), ("d", "delete"),
        ("w", "net worth"), ("q", "quit"),
    ]);
}

fn render_header(frame: &mut Frame, area: Rect, title: &str) {
    let block = Paragraph::new(title)
        .style(Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_BORDER))
            .bg(C_SURFACE));
    frame.render_widget(block, area);
}

fn render_accounts_table(frame: &mut Frame, area: Rect, app: &App) {
    let header_cells = ["ID", "Name", "Type", "Currency", "Balance", "Status"]
        .iter()
        .map(|h| Cell::from(*h).style(
            Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)
        ));
    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows = app.accounts.iter().enumerate().map(|(i, a)| {
        let is_selected = i == app.selected;
        let row_style = if is_selected {
            Style::default().bg(C_HIGHLIGHT)
        } else {
            Style::default().bg(C_BG)
        };

        let balance_style = if a.is_debt {
            Style::default().fg(C_RED)
        } else {
            Style::default().fg(C_GREEN)
        };
        let balance_str = if a.is_debt {
            format!("-{}", a.balance)
        } else {
            a.balance.clone()
        };

        let status = if a.is_overdue {
            Span::styled("⚠ OVERDUE", Style::default().fg(C_YELLOW).add_modifier(Modifier::BOLD))
        } else if a.is_debt {
            Span::styled("debt", Style::default().fg(C_DIM))
        } else {
            Span::styled("ok", Style::default().fg(C_GREEN))
        };

        // Truncate account_id to 8 chars for display
        let short_id = a.account_id.chars().take(8).collect::<String>();

        Row::new(vec![
            Cell::from(short_id).style(Style::default().fg(C_DIM)),
            Cell::from(a.account_name.clone()).style(Style::default().fg(C_TEXT)),
            Cell::from(a.account_type).style(Style::default().fg(C_DIM)),
            Cell::from(a.currency.clone()).style(Style::default().fg(C_DIM)),
            Cell::from(balance_str).style(balance_style),
            Cell::from(Text::from(Line::from(status))),
        ])
        .style(row_style)
        .height(1)
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(10),   // ID
            Constraint::Min(20),      // Name
            Constraint::Length(16),   // Type
            Constraint::Length(8),    // Currency
            Constraint::Length(16),   // Balance
            Constraint::Length(12),   // Status
        ],
    )
    .header(header)
    .block(Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(C_BORDER))
        .bg(C_SURFACE)
        .padding(Padding::horizontal(1)))
    .row_highlight_style(Style::default().bg(C_HIGHLIGHT));

    let mut state = TableState::default().with_selected(Some(app.selected));
    frame.render_stateful_widget(table, area, &mut state);

    if app.accounts.is_empty() {
        let hint = Paragraph::new("No accounts yet.  Press  n  to add your first account.")
            .alignment(Alignment::Center)
            .style(Style::default().fg(C_DIM));
        let inner = inner_centered(40, 1, area);
        frame.render_widget(hint, inner);
    }
}

fn render_footer(frame: &mut Frame, area: Rect, hints: &[(&str, &str)]) {
    let mut spans: Vec<Span> = Vec::new();
    for (i, (key, desc)) in hints.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  ", Style::default()));
        }
        spans.push(Span::styled(
            format!("[{}]", key),
            Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(
            format!(" {}", desc),
            Style::default().fg(C_DIM),
        ));
    }
    let footer = Paragraph::new(Line::from(spans))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_BORDER))
            .bg(C_SURFACE));
    frame.render_widget(footer, area);
}

// ── Net worth screen ──────────────────────────────────────────────────────────

fn render_net_worth(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    render_header(frame, chunks[0], "  netflow — net worth");

    let rows = app.net_worth.iter().map(|r| {
        let nw_style = if r.is_deficit {
            Style::default().fg(C_RED)
        } else {
            Style::default().fg(C_GREEN)
        };
        Row::new(vec![
            Cell::from(r.currency.clone()).style(Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD)),
            Cell::from(r.total_assets.clone()).style(Style::default().fg(C_GREEN)),
            Cell::from(r.total_debts.clone()).style(Style::default().fg(C_RED)),
            Cell::from(r.net_worth.clone()).style(nw_style.add_modifier(Modifier::BOLD)),
        ]).height(1)
    });

    let header = Row::new(["Currency", "Total Assets", "Total Debts", "Net Worth"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD))))
        .height(1).bottom_margin(1);

    let table = Table::new(rows, [
        Constraint::Length(10),
        Constraint::Length(20),
        Constraint::Length(20),
        Constraint::Length(20),
    ])
    .header(header)
    .block(Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(C_BORDER))
        .bg(C_SURFACE)
        .padding(Padding::uniform(1)));

    frame.render_widget(table, chunks[1]);
    render_footer(frame, chunks[2], &[("Esc/q", "back to accounts")]);
}

// ── Add account menu ──────────────────────────────────────────────────────────

fn render_add_menu(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let popup = centered_rect(40, 60, area);

    frame.render_widget(Clear, popup);

    let items: Vec<ListItem> = ACCOUNT_TYPES.iter().enumerate().map(|(i, t)| {
        let style = if i == app.account_type_cursor {
            Style::default().fg(Color::White).bg(C_HIGHLIGHT).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(C_TEXT)
        };
        ListItem::new(format!("  {}  ", t)).style(style)
    }).collect();

    let list = List::new(items)
        .block(Block::default()
            .title(" Select Account Type ")
            .title_style(Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_ACCENT))
            .bg(C_SURFACE)
            .padding(Padding::vertical(1)));

    let mut state = ListState::default().with_selected(Some(app.account_type_cursor));
    frame.render_stateful_widget(list, popup, &mut state);
}

// ── Form overlay ──────────────────────────────────────────────────────────────

fn render_form_overlay(frame: &mut Frame, app: &App) {
    let Some(form) = &app.form else { return };
    let area = frame.area();

    let height = (form.fields.len() as u16) * 3 + 6;
    let popup = centered_rect_fixed(60, height.min(area.height - 4), area);

    frame.render_widget(Clear, popup);

    let block = Block::default()
        .title(format!(" {} ", form.title))
        .title_style(Style::default().fg(C_ACCENT).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(C_ACCENT))
        .bg(C_SURFACE);

    frame.render_widget(block.clone(), popup);
    let inner = block.inner(popup);

    let field_count = form.fields.len();
    let mut constraints: Vec<Constraint> = (0..field_count)
        .map(|_| Constraint::Length(3))
        .collect();
    // Error row + hint row
    constraints.push(Constraint::Length(1));
    constraints.push(Constraint::Length(1));

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    for (i, field) in form.fields.iter().enumerate() {
        let is_focused = i == form.focused;
        let border_style = if is_focused {
            Style::default().fg(C_ACCENT)
        } else {
            Style::default().fg(C_BORDER)
        };
        let display = if field.value.is_empty() {
            Span::styled(field.placeholder, Style::default().fg(C_DIM))
        } else {
            Span::styled(field.value.as_str(), Style::default().fg(C_TEXT))
        };
        let widget = Paragraph::new(Line::from(vec![display]))
            .block(Block::default()
                .title(format!(" {} ", field.label))
                .title_style(Style::default().fg(if is_focused { C_ACCENT } else { C_DIM }))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style));
        frame.render_widget(widget, rows[i]);
    }

    // Error line
    if let Some(err) = &form.error {
        let err_widget = Paragraph::new(err.as_str())
            .style(Style::default().fg(C_RED));
        frame.render_widget(err_widget, rows[field_count]);
    }

    // Hint line
    let hint = Paragraph::new("[Tab] next field   [Shift+Tab] prev   [Enter] submit   [Esc] cancel")
        .style(Style::default().fg(C_DIM));
    frame.render_widget(hint, rows[field_count + 1]);
}

// ── Confirm dialog ────────────────────────────────────────────────────────────

fn render_confirm_dialog(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let popup = centered_rect_fixed(50, 7, area);
    frame.render_widget(Clear, popup);

    let name = app.selected_account()
        .map(|a| a.account_name.as_str())
        .unwrap_or("this account");

    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Remove  ", Style::default().fg(C_TEXT)),
            Span::styled(name, Style::default().fg(C_RED).add_modifier(Modifier::BOLD)),
            Span::styled("?", Style::default().fg(C_TEXT)),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "[y] confirm    [n / Esc] cancel",
            Style::default().fg(C_DIM),
        )),
    ];

    let block = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default()
            .title(" Confirm Removal ")
            .title_style(Style::default().fg(C_RED).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(C_RED))
            .bg(C_SURFACE));

    frame.render_widget(block, popup);
}

// ── Status overlay ────────────────────────────────────────────────────────────

fn render_status_overlay(frame: &mut Frame, message: &str, is_error: bool) {
    let area = frame.area();
    let popup = centered_rect_fixed(50, 5, area);
    frame.render_widget(Clear, popup);

    let (colour, title) = if is_error {
        (C_RED, " Error ")
    } else {
        (C_GREEN, " Done ")
    };

    let block = Paragraph::new(Text::from(vec![
        Line::from(""),
        Line::from(Span::styled(message, Style::default().fg(colour))),
    ]))
    .alignment(Alignment::Center)
    .block(Block::default()
        .title(title)
        .title_style(Style::default().fg(colour).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(colour))
        .bg(C_SURFACE));

    frame.render_widget(block, popup);
}

// ── Layout helpers ────────────────────────────────────────────────────────────

/// Return a centred rect as a percentage of the parent.
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Return a centred rect with fixed width and height in terminal cells.
fn centered_rect_fixed(width: u16, height: u16, r: Rect) -> Rect {
    let x = r.x + r.width.saturating_sub(width) / 2;
    let y = r.y + r.height.saturating_sub(height) / 2;
    Rect::new(x, y, width.min(r.width), height.min(r.height))
}

/// Return a centred rect with fixed width and height inside parent.
fn inner_centered(width: u16, height: u16, r: Rect) -> Rect {
    centered_rect_fixed(width, height, r)
}
