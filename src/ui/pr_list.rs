use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::types::PullRequest;

/// A row in the display list (repo header, PR, or spacer).
///
/// See: src/components/PRList.tsx — DisplayRow interface
#[derive(Debug, Clone)]
pub enum DisplayRow {
    RepoHeader(String),
    Pr { pr: PullRequest, flat_index: usize },
    Spacer,
}

/// Build display rows from a flat list of PRs, grouped by repository.
///
/// See: src/components/PRList.tsx — buildDisplayRows()
pub fn build_display_rows(prs: &[PullRequest]) -> Vec<DisplayRow> {
    let mut rows = Vec::new();
    let mut current_repo = "";
    let mut flat_index = 0;

    for pr in prs {
        let repo = &pr.repository.name_with_owner;
        if repo != current_repo {
            if !current_repo.is_empty() {
                rows.push(DisplayRow::Spacer);
            }
            current_repo = repo;
            rows.push(DisplayRow::RepoHeader(repo.clone()));
        }
        rows.push(DisplayRow::Pr {
            pr: pr.clone(),
            flat_index,
        });
        flat_index += 1;
    }

    rows
}

/// Calculate the visible viewport range.
///
/// See: src/components/PRList.tsx — getViewport()
pub fn get_viewport(
    rows: &[DisplayRow],
    selected_index: usize,
    max_visible: usize,
) -> (usize, usize) {
    let selected_row = rows.iter().position(
        |row| matches!(row, DisplayRow::Pr { flat_index, .. } if * flat_index == selected_index),
    );

    let Some(selected_row) = selected_row else {
        return (0, rows.len().min(max_visible));
    };

    if rows.len() <= max_visible {
        return (0, rows.len());
    };

    let content_max = max_visible.saturating_sub(2);
    let half = content_max / 2;
    let mut start = selected_row.saturating_sub(half);
    let mut end = start + content_max;

    if end > rows.len() {
        end = rows.len();
        start = end.saturating_sub(content_max);
    }

    if start == 0 {
        end = rows.len().min(end + 1);
    }
    if end == rows.len() {
        start = start.saturating_sub(1);
    }

    (start, end)
}

/// Render the PR list (or loading/empty state).
///
/// See: src/components/PRList.tsx — PRList()
pub fn render(
    frame: &mut Frame,
    area: Rect,
    prs: &[PullRequest],
    selected_index: usize,
    empty_message: &str,
    loading: bool,
) {
    let max_visible = area.height as usize;

    if loading && prs.is_empty() {
        let p = Paragraph::new(Line::from(Span::styled(
            "Loading...",
            Style::default().fg(Color::DarkGray),
        )));
        frame.render_widget(p, area);
        return;
    }

    if prs.is_empty() {
        let p = Paragraph::new(Line::from(Span::styled(
            empty_message.to_string(),
            Style::default().fg(Color::DarkGray),
        )));
        frame.render_widget(p, area);
        return;
    }

    let display_rows = build_display_rows(prs);
    let (start, end) = get_viewport(&display_rows, selected_index, max_visible);
    let visible = &display_rows[start..end];

    let mut lines: Vec<Line> = Vec::new();

    if start > 0 {
        lines.push(Line::from(Span::styled(
            "...",
            Style::default().fg(Color::DarkGray),
        )));
    }

    for row in visible {
        match row {
            DisplayRow::RepoHeader(repo) => {
                lines.push(Line::from(Span::styled(
                    repo.clone(),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )));
            }
            DisplayRow::Pr { pr, flat_index } => {
                lines.push(super::pr_row::render_pr_row(
                    pr,
                    *flat_index == selected_index,
                    area.width,
                ));
            }
            DisplayRow::Spacer => {
                lines.push(Line::from(""));
            }
        }
    }

    if end < display_rows.len() {
        lines.push(Line::from(Span::styled(
            "...",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;
    use chrono::{Duration, Utc};
    use ratatui::{Terminal, backend::TestBackend};

    fn make_pr(repo: &str, title: &str) -> PullRequest {
        let t = Utc::now() - Duration::hours(3);
        PullRequest {
            title: title.to_string(),
            url: "u".to_string(),
            number: 1,
            state: PrState::Open,
            is_draft: false,
            created_at: t.to_rfc3339(),
            updated_at: t.to_rfc3339(),
            repository: Repository {
                name_with_owner: repo.to_string(),
            },
            author: Author {
                login: "alice".to_string(),
            },
            labels: Labels { nodes: vec![] },
            review_decision: None,
            review_requests: CountNode { total_count: 0 },
            reviews: CountNode { total_count: 0 },
            commits: CommitNodes { nodes: vec![] },
        }
    }

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
        prs: &[PullRequest],
        selected_index: usize,
        empty_message: &str,
        loading: bool,
    ) -> String {
        let backend = TestBackend::new(80, 10);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                render(
                    frame,
                    frame.area(),
                    prs,
                    selected_index,
                    empty_message,
                    loading,
                );
            })
            .unwrap();
        buffer_to_string(terminal.backend().buffer())
    }

    fn assert_snapshot_with_time_filter(name: &str, output: String) {
        let mut settings = insta::Settings::clone_current();
        settings.add_filter(r"\d+mo|\d+[mhd]", "[T]");
        settings.bind(|| {
            insta::assert_snapshot!(name, output);
        });
    }

    // -- build_display_rows --

    #[test]
    fn build_display_rows_empty() {
        let rows = build_display_rows(&[]);
        assert!(rows.is_empty());
    }

    #[test]
    fn build_display_rows_single_repo() {
        let prs = vec![make_pr("a/a", "t"), make_pr("a/a", "t")];
        let rows = build_display_rows(&prs);

        assert_eq!(rows.len(), 3);
        assert!(matches!(&rows[0], DisplayRow::RepoHeader(r) if r == "a/a"));
        assert!(matches!(&rows[1], DisplayRow::Pr { flat_index: 0, .. }));
        assert!(matches!(&rows[2], DisplayRow::Pr { flat_index: 1, .. }));
    }

    #[test]
    fn build_display_rows_multiple_repos_with_spacers() {
        let prs = vec![
            make_pr("a/a", "t"),
            make_pr("b/b", "t"),
            make_pr("b/b", "t"),
        ];
        let rows = build_display_rows(&prs);

        assert_eq!(rows.len(), 6);
        assert!(matches!(&rows[0], DisplayRow::RepoHeader(r) if r == "a/a"));
        assert!(matches!(&rows[1], DisplayRow::Pr { flat_index: 0, .. }));
        assert!(matches!(&rows[2], DisplayRow::Spacer));
        assert!(matches!(&rows[3], DisplayRow::RepoHeader(r) if r == "b/b"));
        assert!(matches!(&rows[4], DisplayRow::Pr { flat_index: 1, .. }));
        assert!(matches!(&rows[5], DisplayRow::Pr { flat_index: 2, .. }));
    }

    #[test]
    fn build_display_rows_flat_index_is_sequential() {
        let prs = vec![
            make_pr("a/a", "t"),
            make_pr("b/b", "t"),
            make_pr("c/c", "t"),
        ];
        let rows = build_display_rows(&prs);

        let indices: Vec<usize> = rows
            .iter()
            .filter_map(|r| match r {
                DisplayRow::Pr { flat_index, .. } => Some(*flat_index),
                _ => None,
            })
            .collect();
        assert_eq!(indices, vec![0, 1, 2]);
    }

    // -- get_viewport --

    #[test]
    fn get_viewport_all_rows_fit() {
        let prs = vec![make_pr("a/a", "t"), make_pr("a/a", "t")];
        let rows = build_display_rows(&prs);
        let (start, end) = get_viewport(&rows, 0, 10);
        assert_eq!((start, end), (0, 3));
    }

    #[test]
    fn get_viewport_selected_at_top() {
        let prs: Vec<_> = (0..10).map(|_| make_pr("a/a", "t")).collect();
        let rows = build_display_rows(&prs);
        let (start, end) = get_viewport(&rows, 0, 5);
        assert_eq!(start, 0);
        assert!(end <= rows.len());
        assert!(end > start);
    }

    #[test]
    fn get_viewport_selected_at_bottom() {
        let prs: Vec<_> = (0..10).map(|_| make_pr("a/a", "t")).collect();
        let rows = build_display_rows(&prs);
        let (start, end) = get_viewport(&rows, 9, 5);
        assert_eq!(end, rows.len());
        assert!(start > 0);
    }

    #[test]
    fn get_viewport_selected_in_middle() {
        let prs: Vec<_> = (0..20).map(|_| make_pr("a/a", "t")).collect();
        let rows = build_display_rows(&prs);
        let (start, end) = get_viewport(&rows, 10, 7);
        assert!(start > 0);
        assert!(end < rows.len());
        assert!(end - start <= 7);
    }

    // -- render snapshots --

    #[test]
    fn snapshot_loading_empty() {
        let output = render_to_string(&[], 0, "No review requests", true);
        insta::assert_snapshot!(output);
    }

    #[test]
    fn snapshot_empty_not_loading() {
        let output = render_to_string(&[], 0, "No review requests", false);
        insta::assert_snapshot!(output);
    }

    #[test]
    fn snapshot_single_repo() {
        let prs = vec![
            make_pr("owner/repo", "first PR"),
            make_pr("owner/repo", "second PR"),
        ];
        let output = render_to_string(&prs, 0, "", false);
        assert_snapshot_with_time_filter("snapshot_single_repo", output);
    }

    #[test]
    fn snapshot_multiple_repos() {
        let prs = vec![
            make_pr("owner/a", "PR in a"),
            make_pr("owner/b", "PR in b"),
            make_pr("owner/b", "another PR in b"),
        ];
        let output = render_to_string(&prs, 1, "", false);
        assert_snapshot_with_time_filter("snapshot_multiple_repos", output);
    }

    #[test]
    fn snapshot_long_title_truncated() {
        let prs = vec![make_pr(
            "owner/repo",
            "this is an extremely long pull request title that should be truncated to fit within the viewport width",
        )];
        let output = render_to_string(&prs, 0, "", false);
        assert_snapshot_with_time_filter("snapshot_long_title_truncated", output);
    }

    #[test]
    fn snapshot_scrolled_viewport_top_indicator() {
        // Many PRs across many repos → forces "..." indicator at top
        let prs: Vec<_> = (0..15)
            .map(|i| make_pr(&format!("owner/repo-{i}"), "title"))
            .collect();
        // Select near the bottom so upper rows are hidden
        let output = render_to_string(&prs, 14, "", false);
        assert_snapshot_with_time_filter("snapshot_scrolled_viewport_top_indicator", output);
    }

    #[test]
    fn snapshot_scrolled_viewport_bottom_indicator() {
        // Many PRs → forces "..." indicator at bottom
        let prs: Vec<_> = (0..15)
            .map(|i| make_pr(&format!("owner/repo-{i}"), "title"))
            .collect();
        // Select near the top so lower rows are hidden
        let output = render_to_string(&prs, 0, "", false);
        assert_snapshot_with_time_filter("snapshot_scrolled_viewport_bottom_indicator", output);
    }
}
