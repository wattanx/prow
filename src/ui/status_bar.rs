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
        Span::styled(keys.to_string(), Style::default().fg(Color::DarkGray)),
        Span::raw(spacer),
        Span::styled(right, Style::default().fg(Color::DarkGray)),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1));
    let bar = Paragraph::new(line).block(block);
    frame.render_widget(bar, area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend};

    fn buffer_to_string(buffer: &ratatui::buffer::Buffer) -> String {
        let mut s = String::new();
        for y in 0..buffer.area.height {
            for x in 0..buffer.area.width {
                s.push_str(buffer[(x, y)].symbol());
            }
            s.push('\n');
        }
        s
    }

    fn render_to_string(mode: AppMode, sort_order: SortOrder, loading: bool) -> String {
        let backend = TestBackend::new(100, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                render(frame, frame.area(), mode, sort_order, None, loading);
            })
            .unwrap();
        buffer_to_string(terminal.backend().buffer())
    }

    #[test]
    fn snapshot_list_mode_loading() {
        insta::assert_snapshot!(render_to_string(AppMode::List, SortOrder::Newest, true));
    }

    #[test]
    fn snapshot_list_mode_newest() {
        insta::assert_snapshot!(render_to_string(AppMode::List, SortOrder::Newest, false));
    }

    #[test]
    fn snapshot_list_mode_oldest() {
        insta::assert_snapshot!(render_to_string(AppMode::List, SortOrder::Oldest, false));
    }

    #[test]
    fn snapshot_filter_mode() {
        insta::assert_snapshot!(render_to_string(AppMode::Filter, SortOrder::Newest, true));
    }
}
