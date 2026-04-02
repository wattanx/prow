use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::app::SectionCounts;
use crate::types::SectionType;

/// Render section tabs: "new (3)  stale (1)  all (5)  authored (2)"
/// Active section is highlighted in blue with inverse style.
///
/// See: src/components/SectionList.tsx
pub fn render(frame: &mut Frame, area: Rect, active: SectionType, counts: &SectionCounts) {
    todo!("Render section tabs with counts, highlight active section")
}
