use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph},
};

use crate::app::SectionCounts;
use crate::types::SectionType;

/// Render section tabs: "new (3)  stale (1)  all (5)  authored (2)"
/// Active section is highlighted in blue with inverse style.
///
/// See: src/components/SectionList.tsx
pub fn render(frame: &mut Frame, area: Rect, active: SectionType, counts: &SectionCounts) {
    let spans: Vec<Span> = SectionType::ALL_SECTIONS
        .iter()
        .map(|section| {
            let label = format!(" {} ({}) ", section.label(), counts.get(*section));

            if *section == active {
                Span::styled(
                    label,
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::BOLD | Modifier::REVERSED),
                )
            } else {
                Span::styled(label, Style::default().fg(Color::DarkGray))
            }
        })
        .collect();

    let line = Line::from(spans);
    let block = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1));
    let tabs = Paragraph::new(line).block(block);
    frame.render_widget(tabs, area);
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

    fn render_to_string(
        width: u16,
        height: u16,
        active: SectionType,
        counts: &SectionCounts,
    ) -> String {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                render(frame, frame.area(), active, counts);
            })
            .unwrap();
        buffer_to_string(terminal.backend().buffer())
    }

    #[test]
    fn snapshot_active_new() {
        let counts = SectionCounts {
            new: 3,
            stale: 1,
            all: 5,
            authored: 2,
        };
        let output = render_to_string(60, 3, SectionType::New, &counts);
        insta::assert_snapshot!(output);
    }

    #[test]
    fn snapshot_active_all() {
        let counts = SectionCounts {
            new: 0,
            stale: 0,
            all: 10,
            authored: 0,
        };
        let output = render_to_string(60, 3, SectionType::All, &counts);
        insta::assert_snapshot!(output);
    }

    #[test]
    fn snapshot_all_zero_counts() {
        let counts = SectionCounts {
            new: 0,
            stale: 0,
            all: 0,
            authored: 0,
        };
        let output = render_to_string(60, 3, SectionType::New, &counts);
        insta::assert_snapshot!(output);
    }
}
