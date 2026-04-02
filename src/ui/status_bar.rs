use chrono::{DateTime, Utc};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::types::{AppMode, SortOrder};

/// Render the status bar at the bottom.
///
/// List mode:  "j/k move  h/l section  g/G top/end  ⏎ open  s sort  f filter  r refresh  q quit"
/// Filter mode: "↑↓/jk move  Space toggle  ⏎ apply  Esc cancel"
///
/// Right side: sort order + last updated time (or "Loading...")
///
/// See: src/components/StatusBar.tsx
pub fn render(
    frame: &mut Frame,
    area: Rect,
    mode: AppMode,
    sort_order: SortOrder,
    last_updated: Option<DateTime<Utc>>,
    loading: bool,
) {
    todo!("Render status bar with keybindings, sort order, and update time")
}
