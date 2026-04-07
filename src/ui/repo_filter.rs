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
        all_repos: &[String],
        selected_repos: &BTreeSet<String>,
        cursor_index: usize,
    ) -> String {
        let backend = TestBackend::new(60, 10);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                render(frame, frame.area(), all_repos, selected_repos, cursor_index);
            })
            .unwrap();
        buffer_to_string(terminal.backend().buffer())
    }

    #[test]
    fn snapshot_all_state_cursor_at_top() {
        let repos = vec![
            "owner/a".to_string(),
            "owner/b".to_string(),
            "owner/c".to_string(),
        ];
        let selected = BTreeSet::new();
        insta::assert_snapshot!(render_to_string(&repos, &selected, 0));
    }

    #[test]
    fn snapshot_partial_selection() {
        let repos = vec![
            "owner/a".to_string(),
            "owner/b".to_string(),
            "owner/c".to_string(),
        ];
        let mut selected = BTreeSet::new();
        selected.insert("owner/b".to_string());
        insta::assert_snapshot!(render_to_string(&repos, &selected, 2));
    }

    #[test]
    fn snapshot_empty_repos() {
        let repos: Vec<String> = vec![];
        let selected = BTreeSet::new();
        insta::assert_snapshot!(render_to_string(&repos, &selected, 0));
    }

    #[test]
    fn snapshot_cursor_on_middle_repo() {
        let repos = vec![
            "owner/a".to_string(),
            "owner/b".to_string(),
            "owner/c".to_string(),
        ];
        let selected = BTreeSet::new();
        insta::assert_snapshot!(render_to_string(&repos, &selected, 2));
    }
}
