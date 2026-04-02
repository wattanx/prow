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
    Pr {
        pr: PullRequest,
        flat_index: usize,
    },
    Spacer,
}

/// Build display rows from a flat list of PRs, grouped by repository.
///
/// See: src/components/PRList.tsx — buildDisplayRows()
pub fn build_display_rows(prs: &[PullRequest]) -> Vec<DisplayRow> {
    todo!("Group PRs by repository, insert headers and spacers")
}

/// Calculate the visible viewport range.
///
/// See: src/components/PRList.tsx — getViewport()
pub fn get_viewport(
    rows: &[DisplayRow],
    selected_index: usize,
    max_visible: usize,
) -> (usize, usize) {
    todo!("Calculate start/end indices for viewport scrolling with '...' indicators")
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
    todo!("Build display rows, compute viewport, render visible rows with repo headers")
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Test build_display_rows groups PRs by repository
    // TODO: Test get_viewport with various list sizes and selected indices
}
