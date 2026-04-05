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
    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(Span::styled(
        "Filter by repository",
        Style::default().add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    let is_all_selected = selected_repos.is_empty();
    let all_cursor = if cursor_index == 0 { ">" } else { " " };
    let all_check = if is_all_selected { "x" } else { " " };
    let all_style = if cursor_index == 0 {
        Style::default().fg(Color::Blue)
    } else {
        Style::default()
    };

    lines.push(Line::from(Span::styled(
        format!("{all_cursor} [{all_check}] All"),
        all_style,
    )));

    for (i, repo) in all_repos.iter().enumerate() {
        let item_index = i + 1;
        let cursor = if cursor_index == item_index { ">" } else { " " };
        let is_checked = is_all_selected || selected_repos.contains(repo);
        let check = if is_checked { "x" } else { " " };
        let style = if cursor_index == item_index {
            Style::default().fg(Color::Blue)
        } else {
            Style::default()
        };

        lines.push(Line::from(Span::styled(
            format!("{cursor} [{check}] {repo}"),
            style,
        )));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, area);
}
