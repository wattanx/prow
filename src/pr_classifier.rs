use crate::types::PullRequest;

/// 48 hours in milliseconds.
const STALE_THRESHOLD_MS: i64 = 48 * 60 * 60 * 1000;

/// Filter PRs that are "new": no reviews yet AND updated within 48 hours.
///
/// See: src/hooks/usePRs.ts — classifyNew()
pub fn classify_new(prs: &[PullRequest]) -> Vec<PullRequest> {
    todo!("Filter PRs with no reviews and updated within 48h")
}

/// Filter PRs that are "stale": not updated for 48+ hours.
///
/// See: src/hooks/usePRs.ts — classifyStale()
pub fn classify_stale(prs: &[PullRequest]) -> Vec<PullRequest> {
    todo!("Filter PRs not updated for 48+ hours")
}

#[cfg(test)]
mod tests {
    // TODO: Test classify_new with mock PRs at various ages
    // TODO: Test classify_stale with mock PRs at various ages
}
