use chrono::{DateTime, Local, Utc};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph},
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
    let keys = match mode {
        AppMode::Filter => "↑↓/jk move  Space toggle  ⏎ apply  Esc cancel",
        AppMode::List => {
            "j/k move  h/l section  g/G top/end  ⏎ open  s sort  f filter  r refresh  q quit"
        }
    };

    let time_str = if loading {
        "Loading...".to_string()
    } else {
        match last_updated {
            Some(t) => {
                let local: DateTime<Local> = t.with_timezone(&Local);
                format!("Updated {}", local.format("%-I:%M %p"))
            }
            None => "Updated -".to_string(),
        }
    };

    let right = if mode == AppMode::Filter {
        time_str
    } else {
        let sort_label = match sort_order {
            SortOrder::Newest => "newest first",
            SortOrder::Oldest => "oldest first",
        };
        format!("Sort: {}  {}", sort_label, time_str)
    };

    // Borders::ALL takes 1 col on each side + padding 1 on each side = 4
    let inner_width = (area.width as usize).saturating_sub(4);
    let spacer_width = inner_width.saturating_sub(keys.len() + right.len());
    let spacer = " ".repeat(spacer_width);

    let line = Line::from(vec![
        Span::styled(keys.to_string(), Style::default().fg(Color::Gray)),
        Span::raw(spacer),
        Span::styled(right, Style::default().fg(Color::Gray)),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1));
    let bar = Paragraph::new(line).block(block);
    frame.render_widget(bar, area);
}
