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
