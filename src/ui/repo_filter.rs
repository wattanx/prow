use std::collections::BTreeSet;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

/// Render the repository filter overlay.
///
/// Shows:
///   Filter by repository:
///   > [x] All
///     [x] owner/repo-1
///     [ ] owner/repo-2
///
/// See: src/components/RepoFilter.tsx
pub fn render(
    frame: &mut Frame,
    area: Rect,
    all_repos: &[String],
    selected_repos: &BTreeSet<String>,
    cursor_index: usize,
) {
    todo!("Render filter list with checkboxes and cursor")
}
