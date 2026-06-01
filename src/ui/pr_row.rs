use ratatui::{
    style::{Color, Style},
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
///   None/unknown -> ("-", Color::DarkGray)
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
        None => ("-", Color::DarkGray),
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
        Style::default().fg(Color::DarkGray)
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
    if max == 0 {
        return String::new();
    }

    let width = display_width(s);
    if width <= max {
        return pad_to_width(s.to_string(), width, max);
    }

    let ellipsis = "…";
    let ellipsis_width = display_width(ellipsis);
    let content_width = max.saturating_sub(ellipsis_width);
    let mut out = String::new();
    let mut used_width = 0;

    for ch in s.chars() {
        let mut buf = [0; 4];
        let char_width = display_width(ch.encode_utf8(&mut buf));
        if used_width + char_width > content_width {
            break;
        }

        out.push(ch);
        used_width += char_width;
    }

    out.push_str(ellipsis);
    used_width += ellipsis_width;

    pad_to_width(out, used_width, max)
}

fn display_width(s: &str) -> usize {
    Span::raw(s).width()
}

fn pad_to_width(mut s: String, width: usize, max: usize) -> String {
    if width < max {
        s.push_str(&" ".repeat(max - width));
    }
    s
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use super::*;
    use crate::types::*;

    fn make_pr_with_ci(state: Option<CheckState>) -> PullRequest {
        PullRequest {
            title: "t".to_string(),
            url: "u".to_string(),
            number: 1,
            state: PrState::Open,
            is_draft: false,
            created_at: "2026-04-01T00:00:00Z".to_string(),
            updated_at: "2026-04-01T00:00:00Z".to_string(),
            repository: Repository {
                name_with_owner: "o/r".to_string(),
            },
            author: Author {
                login: "a".to_string(),
            },
            labels: Labels { nodes: vec![] },
            review_decision: None,
            review_requests: CountNode { total_count: 0 },
            reviews: CountNode { total_count: 0 },
            commits: CommitNodes {
                nodes: vec![CommitNode {
                    commit: CommitInfo {
                        status_check_rollup: Some(StatusCheckRollup { state }),
                    },
                }],
            },
        }
    }

    #[test]
    fn format_relative_time_minutes() {
        let t = Utc::now() - Duration::minutes(5);
        assert_eq!(format_relative_time(&t.to_rfc3339()), "5m");
    }

    #[test]
    fn format_relative_time_hours() {
        let t = Utc::now() - Duration::hours(3);
        assert_eq!(format_relative_time(&t.to_rfc3339()), "3h");
    }

    #[test]
    fn format_relative_time_days() {
        let t = Utc::now() - Duration::days(5);
        assert_eq!(format_relative_time(&t.to_rfc3339()), "5d");
    }

    #[test]
    fn format_relative_time_months() {
        let t = Utc::now() - Duration::days(60);
        assert_eq!(format_relative_time(&t.to_rfc3339()), "2mo");
    }

    #[test]
    fn format_relative_time_invalid_returns_dash() {
        assert_eq!(format_relative_time("not a date"), "-");
    }

    #[test]
    fn ci_status_success() {
        let pr = make_pr_with_ci(Some(CheckState::Success));
        assert_eq!(ci_status(&pr), ("✓", Color::Green));
    }

    #[test]
    fn ci_status_failure() {
        let pr = make_pr_with_ci(Some(CheckState::Failure));
        assert_eq!(ci_status(&pr), ("✗", Color::Red));
    }

    #[test]
    fn ci_status_pending() {
        let pr = make_pr_with_ci(Some(CheckState::Pending));
        assert_eq!(ci_status(&pr), ("◌", Color::Yellow));
    }

    #[test]
    fn ci_status_none() {
        let pr = make_pr_with_ci(None);
        assert_eq!(ci_status(&pr), ("-", Color::DarkGray));
    }

    #[test]
    fn truncate_ascii_title() {
        assert_eq!(truncate("abcdef", 4), "abc…");
    }

    #[test]
    fn truncate_multibyte_title_on_char_boundary() {
        let truncated = truncate("日本語の長いタイトルでも文字の途中で切れないようにする", 44);

        assert_eq!(truncated, "日本語の長いタイトルでも文字の途中で切れな… ");
        assert_eq!(display_width(&truncated), 44);
    }

    #[test]
    fn truncate_pads_short_multibyte_title_to_display_width() {
        assert_eq!(truncate("店舗", 6), "店舗  ");
    }

    #[test]
    fn truncate_does_not_cut_multibyte_title_when_display_width_fits() {
        let title = "日本語の長いタイトルでも文字の途中で切れないようにする";
        let truncated = truncate(title, 132);

        assert_eq!(truncated.trim_end(), title);
        assert_eq!(display_width(&truncated), 132);
    }
}
