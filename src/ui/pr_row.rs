use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use crate::types::{CheckState, PullRequest};

/// Format a datetime string as relative time (e.g., "3m", "2h", "5d", "1mo").
///
/// See: src/components/PRRow.tsx — formatRelativeTime()
pub fn format_relative_time(date_str: &str) -> String {
    todo!("Parse ISO 8601 date and compute relative time from now")
}

/// Get CI status icon and color from a PR's commit status.
///
/// See: src/components/PRRow.tsx — getCIStatus()
///   SUCCESS  -> ("✓", Color::Green)
///   FAILURE/ERROR -> ("✗", Color::Red)
///   PENDING/EXPECTED -> ("◌", Color::Yellow)
///   None/unknown -> ("-", Color::Gray)
pub fn ci_status(pr: &PullRequest) -> (&'static str, Color) {
    todo!("Map commit statusCheckRollup to icon and color")
}

/// Build a Line for a single PR row.
///
/// Format: "[>] {ci_icon} {title...} {time} {author}"
///
/// See: src/components/PRRow.tsx — PRRow()
pub fn render_pr_row(pr: &PullRequest, is_selected: bool, width: u16) -> Line<'static> {
    todo!("Build a styled Line with selection indicator, CI icon, title, time, author")
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Test format_relative_time with various time differences
    // TODO: Test ci_status with different check states
    // TODO: Test render_pr_row produces expected spans
}
