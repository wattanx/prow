use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

/// Application actions triggered by keyboard input.
///
/// See: src/app.tsx — useInput() handler (lines 91-143)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    // -- List mode --
    Quit,
    MoveDown,
    MoveUp,
    OpenInBrowser,
    NextSection,
    PrevSection,
    GoToTop,
    GoToBottom,
    ToggleSort,
    EnterFilter,
    Refresh,

    // -- Filter mode --
    FilterMoveDown,
    FilterMoveUp,
    FilterToggle,
    FilterApply,
    FilterCancel,

    // -- System --
    Tick,
}

/// Poll for terminal events and map to Action.
/// Returns None if the event doesn't map to any action.
pub fn poll_event(tick_rate: Duration) -> Result<Option<Action>> {
    if event::poll(tick_rate)? {
        if let Event::Key(key) = event::read()? {
            return Ok(map_key_event(key));
        }
    }
    Ok(Some(Action::Tick))
}

/// Map a key event to an Action.
/// This does NOT know about the current mode — the caller (app.rs)
/// decides which actions are valid in each mode.
fn map_key_event(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Char('j') | KeyCode::Down => Some(Action::MoveDown),
        KeyCode::Char('k') | KeyCode::Up => Some(Action::MoveUp),
        KeyCode::Enter => Some(Action::OpenInBrowser),
        KeyCode::Char('h') => Some(Action::PrevSection),
        KeyCode::Char('l') => Some(Action::NextSection),
        KeyCode::Tab if key.modifiers.contains(KeyModifiers::SHIFT) => Some(Action::PrevSection),
        KeyCode::Tab => Some(Action::NextSection),
        KeyCode::Char('g') => Some(Action::GoToTop),
        KeyCode::Char('G') => Some(Action::GoToBottom),
        KeyCode::Char('s') => Some(Action::ToggleSort),
        KeyCode::Char('f') => Some(Action::EnterFilter),
        KeyCode::Char('r') => Some(Action::Refresh),
        KeyCode::Char(' ') => Some(Action::FilterToggle),
        KeyCode::Esc => Some(Action::FilterCancel),
        _ => None,
    }
}
