use std::{fmt, io, io::Write};

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
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
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
    run_ui_with_message(items, None)
}

pub(crate) fn run_ui_with_message(
    items: Vec<PaletteItem>,
    message: Option<&str>,
) -> Result<Option<PaletteItem>, UiError> {
    let mut cleanup = TerminalCleanup::new();
    cleanup.enable_raw_mode()?;
    let mut stdout = io::stdout();
    cleanup.enter_alternate_screen(&mut stdout)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut query = String::new();
    let mut selected = 0;

    loop {
        let ranked = rank(&query, &items);
        selected = selected.min(ranked.len().saturating_sub(1));
        terminal.draw(|frame| draw(frame, &items, &query, &ranked, selected, message))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key {
                KeyEvent {
                    code: KeyCode::Esc | KeyCode::Enter,
                    ..
                } if message.is_some() => return Ok(None),
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
    message: Option<&str>,
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
                    .fg(Color::Black)
                    .bg(Color::Yellow)
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

    if let Some(message) = message {
        let popup = ratatui::layout::Rect {
            x: frame.area().x + frame.area().width / 8,
            y: frame.area().y + frame.area().height / 3,
            width: frame.area().width * 3 / 4,
            height: 5,
        };
        frame.render_widget(Clear, popup);
        frame.render_widget(
            Paragraph::new(format!("{message}\n\nPress Enter or Escape to close."))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Herdr action failed "),
                )
                .wrap(Wrap { trim: true }),
            popup,
        );
    }
}

struct TerminalCleanup {
    raw_mode: bool,
    alternate_screen: bool,
    cursor_hidden: bool,
}

impl TerminalCleanup {
    fn new() -> Self {
        Self {
            raw_mode: false,
            alternate_screen: false,
            cursor_hidden: false,
        }
    }

    fn enable_raw_mode(&mut self) -> io::Result<()> {
        self.acquire_raw_mode(enable_raw_mode)
    }

    fn acquire_raw_mode(&mut self, acquire: impl FnOnce() -> io::Result<()>) -> io::Result<()> {
        acquire()?;
        self.raw_mode = true;
        Ok(())
    }

    fn enter_alternate_screen(&mut self, writer: &mut impl Write) -> io::Result<()> {
        execute!(writer, EnterAlternateScreen)?;
        self.alternate_screen = true;
        execute!(writer, cursor::Hide)?;
        self.cursor_hidden = true;
        Ok(())
    }

    fn restore_screen(&self, writer: &mut impl Write) -> io::Result<()> {
        if self.alternate_screen {
            execute!(writer, LeaveAlternateScreen)?;
        }
        if self.cursor_hidden {
            execute!(writer, cursor::Show)?;
        }
        Ok(())
    }
}

impl Drop for TerminalCleanup {
    fn drop(&mut self) {
        if self.raw_mode {
            let _ = disable_raw_mode();
        }
        let mut stdout = io::stdout();
        let _ = self.restore_screen(&mut stdout);
    }
}

#[cfg(test)]
mod tests {
    use ratatui::{backend::TestBackend, style::Color, Terminal};

    use super::{draw, TerminalCleanup};
    use crate::{catalog::default_items, rank};

    #[test]
    fn failed_setup_does_not_emit_alternate_screen_cleanup() {
        let mut cleanup = TerminalCleanup::new();
        let mut output = Vec::new();

        let setup_result =
            cleanup.acquire_raw_mode(|| Err(std::io::Error::other("not a terminal")));

        assert!(setup_result.is_err());
        cleanup.restore_screen(&mut output).unwrap();

        assert!(output.is_empty());
    }

    #[test]
    fn selected_result_uses_high_contrast_foreground_and_background() {
        let items = default_items();
        let ranked = rank("", &items);
        let backend = TestBackend::new(80, 20);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|frame| draw(frame, &items, "", &ranked, 0, None))
            .unwrap();

        let selected_title = &terminal.backend().buffer()[(1, 7)];
        assert_eq!(selected_title.fg, Color::Black);
        assert_eq!(selected_title.bg, Color::Yellow);
    }
}
