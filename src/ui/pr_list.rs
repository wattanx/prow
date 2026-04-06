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

    // TODO: Test build_display_rows groups PRs by repository
    // TODO: Test get_viewport with various list sizes and selected indices
}
