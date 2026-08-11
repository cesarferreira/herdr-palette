use std::{fmt, io};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};

use crate::{rank, PaletteItem};

#[derive(Debug)]
pub struct UiError(String);

impl fmt::Display for UiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for UiError {}

impl From<io::Error> for UiError {
    fn from(error: io::Error) -> Self {
        Self(error.to_string())
    }
}

/// Opens the interactive palette and returns the item selected by the user.
pub fn run_ui(items: Vec<PaletteItem>) -> Result<Option<PaletteItem>, UiError> {
    let _cleanup = TerminalCleanup;
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut query = String::new();
    let mut selected = 0;

    loop {
        let ranked = rank(&query, &items);
        selected = selected.min(ranked.len().saturating_sub(1));
        terminal.draw(|frame| draw(frame, &items, &query, &ranked, selected))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key {
                KeyEvent {
                    code: KeyCode::Esc, ..
                } => return Ok(None),
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => {
                    return Ok(ranked.get(selected).map(|index| items[*index].clone()));
                }
                KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                } => {
                    query.pop();
                    selected = 0;
                }
                KeyEvent {
                    code: KeyCode::Char('u'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    query.clear();
                    selected = 0;
                }
                KeyEvent {
                    code: KeyCode::Up, ..
                }
                | KeyEvent {
                    code: KeyCode::Char('p'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => selected = selected.saturating_sub(1),
                KeyEvent {
                    code: KeyCode::Down,
                    ..
                }
                | KeyEvent {
                    code: KeyCode::Char('n'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => selected = (selected + 1).min(ranked.len().saturating_sub(1)),
                KeyEvent {
                    code: KeyCode::Char(character),
                    modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                    ..
                } => {
                    query.push(character);
                    selected = 0;
                }
                _ => {}
            }
        }
    }
}

fn draw(
    frame: &mut ratatui::Frame,
    items: &[PaletteItem],
    query: &str,
    ranked: &[usize],
    selected: usize,
) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(4),
            Constraint::Length(4),
        ])
        .split(frame.area());
    frame.render_widget(
        Paragraph::new("Herdr command palette")
            .block(Block::default().borders(Borders::ALL).title(" Palette "))
            .style(Style::default().add_modifier(Modifier::BOLD)),
        areas[0],
    );
    frame.render_widget(
        Paragraph::new(format!("> {query}"))
            .block(Block::default().borders(Borders::ALL).title(" Search ")),
        areas[1],
    );

    let rows = ranked
        .iter()
        .map(|index| {
            let item = &items[*index];
            let shortcut = item.shortcuts.join("  ");
            ListItem::new(Line::from(vec![
                Span::styled(&item.title, Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("  "),
                Span::styled(shortcut, Style::default().fg(Color::Cyan)),
            ]))
        })
        .collect::<Vec<_>>();
    let mut state = ListState::default();
    state.select((!ranked.is_empty()).then_some(selected));
    frame.render_stateful_widget(
        List::new(rows)
            .block(Block::default().borders(Borders::ALL).title(" Results "))
            .highlight_style(
                Style::default()
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
        areas[2],
        &mut state,
    );

    let detail = ranked
        .get(selected)
        .map(|index| {
            let item = &items[*index];
            Line::from(vec![
                Span::raw(&item.description),
                Span::raw("  "),
                Span::styled(
                    format!("[{}]", item.shortcuts.join("] [")),
                    Style::default().fg(Color::Cyan),
                ),
            ])
        })
        .unwrap_or_else(|| Line::from("No matching commands"));
    frame.render_widget(
        Paragraph::new(detail)
            .block(Block::default().borders(Borders::ALL).title(" Details "))
            .wrap(Wrap { trim: true }),
        areas[3],
    );
}

struct TerminalCleanup;

impl Drop for TerminalCleanup {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let mut stdout = io::stdout();
        let _ = execute!(stdout, LeaveAlternateScreen, cursor::Show);
    }
}
