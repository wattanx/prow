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
    let Ok(date) = date_str.parse::<chrono::DateTime<chrono::Utc>>() else {
        return "-".to_string();
    };

    let diff = chrono::Utc::now() - date;
    let minutes = diff.num_minutes();

    if minutes < 60 {
        format!("{minutes}m")
    } else if minutes < 60 * 24 {
        format!("{}h", minutes / 60)
    } else if minutes < 60 * 24 * 30 {
        format!("{}d", minutes / (60 * 24))
    } else {
        format!("{}mo", minutes / (60 * 24 * 30))
    }
}

/// Get CI status icon and color from a PR's commit status.
///
/// See: src/components/PRRow.tsx — getCIStatus()
///   SUCCESS  -> ("✓", Color::Green)
///   FAILURE/ERROR -> ("✗", Color::Red)
///   PENDING/EXPECTED -> ("◌", Color::Yellow)
///   None/unknown -> ("-", Color::Gray)
pub fn ci_status(pr: &PullRequest) -> (&'static str, Color) {
    let state = pr
        .commits
        .nodes
        .first()
        .and_then(|n| n.commit.status_check_rollup.as_ref())
        .and_then(|s| s.state.as_ref());

    match state {
        Some(CheckState::Success) => ("✓", Color::Green),
        Some(CheckState::Failure | CheckState::Error) => ("✗", Color::Red),
        Some(CheckState::Pending | CheckState::Expected) => ("◌", Color::Yellow),
        None => ("-", Color::Gray),
    }
}

/// Build a Line for a single PR row.
///
/// Format: "[>] {ci_icon} {title...} {time} {author}"
///
/// See: src/components/PRRow.tsx — PRRow()
pub fn render_pr_row(pr: &PullRequest, is_selected: bool, width: u16) -> Line<'static> {
    let (icon, icon_color) = ci_status(pr);
    let time = format_relative_time(&pr.updated_at);
    let author = &pr.author.login;

    let selector = if is_selected { "> " } else { "  " };
    let style = if is_selected {
        Style::default().bg(Color::DarkGray)
    } else {
        Style::default()
    };
    let dim = if is_selected {
        Style::default().bg(Color::DarkGray)
    } else {
        Style::default().fg(Color::Gray)
    };

    let fixed_width = 25;
    let title_width = (width as usize).saturating_sub(fixed_width);
    let title = truncate(&pr.title, title_width);

    Line::from(vec![
        Span::styled(selector.to_string(), style),
        Span::styled(
            format!("{icon} "),
            Style::default()
                .fg(icon_color)
                .bg(style.bg.unwrap_or(Color::Reset)),
        ),
        Span::styled(title, style),
        Span::styled(format!("{:>5}", time), dim),
        Span::styled(format!("{:>16}", author), dim),
    ])
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        format!("{:width$}", s, width = max)
    } else {
        format!("{}…", &s[..max.saturating_sub(1)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Test format_relative_time with various time differences
    // TODO: Test ci_status with different check states
    // TODO: Test render_pr_row produces expected spans
}
