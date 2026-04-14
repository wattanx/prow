use crate::types::PullRequest;
use chrono::Utc;

/// 48 hours in milliseconds.
const STALE_THRESHOLD_MS: i64 = 48 * 60 * 60 * 1000;

/// Filter PRs that are "new": no reviews yet AND updated within 48 hours.
///
/// See: src/hooks/usePRs.ts — classifyNew()
pub fn classify_new(prs: &[PullRequest]) -> Vec<PullRequest> {
    prs.iter()
        .filter(|pr| {
            let has_no_reviews = pr.reviews.total_count == 0;
            let now = Utc::now();
            let Ok(updated) = pr.updated_at.parse::<chrono::DateTime<Utc>>() else {
                return false;
            };

            let age_ms = (now - updated).num_milliseconds();
            let is_recent = age_ms < STALE_THRESHOLD_MS;
            has_no_reviews && is_recent
        })
        .cloned()
        .collect()
}

/// Filter PRs that are "stale": not updated for 48+ hours.
///
/// See: src/hooks/usePRs.ts — classifyStale()
pub fn classify_stale(prs: &[PullRequest]) -> Vec<PullRequest> {
    prs.iter()
        .filter(|pr| {
            let now = Utc::now();
            let Ok(updated) = pr.updated_at.parse::<chrono::DateTime<Utc>>() else {
                return false;
            };

            let age_ms = (now - updated).num_milliseconds();
            age_ms >= STALE_THRESHOLD_MS
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{pr_classifier::classify_new, types::*};
    use chrono::{Duration, Utc};

    fn make_pr(updated_at: chrono::DateTime<Utc>, review_count: u64) -> PullRequest {
        PullRequest {
            title: "test".to_string(),
            url: "https://example.com".to_string(),
            number: 1,
            state: PrState::Open,
            is_draft: false,
            created_at: updated_at.to_rfc3339(),
            updated_at: updated_at.to_rfc3339(),
            repository: Repository {
                name_with_owner: "owner/repo".to_string(),
            },
            author: Author {
                login: "alice".to_string(),
            },
            labels: Labels { nodes: vec![] },
            review_decision: None,
            review_requests: CountNode { total_count: 0 },
            reviews: CountNode {
                total_count: review_count,
            },
            commits: CommitNodes { nodes: vec![] },
        }
    }

    #[test]
    fn classify_new_includes_recent_pr_with_no_reviews() {
        let now = Utc::now();
        let prs = vec![
            make_pr(now - Duration::hours(1), 0), // 1h ago, 0 reviews -> include
            make_pr(now - Duration::hours(1), 2), // 1h ago, 2 reviews -> exclude
            make_pr(now - Duration::hours(50), 0), // 50h ago, -> exclude (stale)
        ];

        let result = classify_new(&prs);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn classify_new_empty_input() {
        let result = classify_new(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn classify_stale_includes_old_prs() {
        let now = Utc::now();
        let prs = vec![
            make_pr(now - Duration::hours(50), 0), // 50h → stale
            make_pr(now - Duration::hours(1), 0),  // 1h → not stale
            make_pr(now - Duration::hours(48), 0), // exactly 48h → stale
        ];

        let result = classify_stale(&prs);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn classify_stale_empty_input() {
        let result = classify_stale(&[]);
        assert!(result.is_empty());
    }
}
