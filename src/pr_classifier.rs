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
    // TODO: Test classify_new with mock PRs at various ages
    // TODO: Test classify_stale with mock PRs at various ages
}
