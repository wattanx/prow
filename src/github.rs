use anyhow::Result;
use tokio::process::Command;

use crate::types::PullRequest;

/// GraphQL search query template.
/// See: src/lib/github.ts — SEARCH_QUERY
const SEARCH_QUERY: &str = r#"
    query($searchQuery: String!, $first: Int!, $after: String) {
      search(query: $searchQuery, type: ISSUE, first: $first, after: $after) {
        issueCount
        pageInfo {
          hasNextPage
          endCursor
        }
        nodes {
          ... on PullRequest {
            title
            url
            number
            state
            isDraft
            createdAt
            updatedAt
            repository { nameWithOwner }
            author { login }
            labels(first: 10) { nodes { name color } }
            reviewDecision
            reviewRequests { totalCount }
            reviews(first: 1) { totalCount }
            commits(last: 1) { nodes { commit { statusCheckRollup { state } } } }
          }
        }
      }
    }
  "#;

/// Trait for GitHub API access (allows mocking in tests).
pub trait GitHubClient {
    fn fetch_created_prs(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<PullRequest>>> + Send;
    fn fetch_review_requested_prs(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<PullRequest>>> + Send;
}

/// Real implementation that shells out to `gh api graphql`.
pub struct GhCliClient;

impl GhCliClient {
    pub fn new() -> Self {
        Self
    }
}

impl GitHubClient for GhCliClient {
    /// Fetch PRs authored by the current user.
    /// Query: "author:@me is:pr is:open sort:updated-desc"
    ///
    /// See: src/lib/github.ts — fetchCreatedPRs()
    async fn fetch_created_prs(&self) -> Result<Vec<PullRequest>> {
        fetch_all_pages("author:@me is:pr is:open sort:updated-desc").await
    }

    /// Fetch PRs where review is requested from the current user.
    /// Query: "review-requested:@me is:pr is:open sort:updated-desc"
    ///
    /// See: src/lib/github.ts — fetchReviewRequestedPRs()
    async fn fetch_review_requested_prs(&self) -> Result<Vec<PullRequest>> {
        fetch_all_pages("review-requested:@me is:pr is:open sort:updated-desc").await
    }
}

/// Execute a GraphQL query via `gh api graphql` and parse the response.
///
/// See: src/lib/github.ts — ghGraphql()
async fn gh_graphql(
    query: &str,
    search_query: &str,
    first: u32,
    after: Option<&str>,
) -> Result<SearchResponse> {
    let mut args = vec![
        "api".to_string(),
        "graphql".to_string(),
        "-f".to_string(),
        format!("query={query}"),
        "-f".to_string(),
        format!("searchQuery={search_query}"),
        "-F".to_string(),
        format!("first={first}"),
    ];

    if let Some(cursor) = after {
        args.push("-f".to_string());
        args.push(format!("after={cursor}"));
    }

    let output = Command::new("gh").args(&args).output().await?;

    let stdout = String::from_utf8(output.stdout)?;
    let response: SearchResponse = serde_json::from_str(&stdout)?;
    Ok(response)
}

/// Fetch all pages of a search query.
///
/// See: src/lib/github.ts — fetchAllPages()
async fn fetch_all_pages(search_query: &str) -> Result<Vec<PullRequest>> {
    let mut all_prs = Vec::new();
    let mut after: Option<String> = None;

    loop {
        let response = gh_graphql(SEARCH_QUERY, search_query, 50, after.as_deref()).await?;

        all_prs.extend(response.data.search.nodes);

        if !response.data.search.page_info.has_next_page {
            break;
        }
        after = response.data.search.page_info.end_cursor;
    }

    Ok(all_prs)
}

// -- Response types for gh API JSON parsing --

#[derive(Debug, serde::Deserialize)]
pub struct SearchResponse {
    pub data: SearchData,
}

#[derive(Debug, serde::Deserialize)]
pub struct SearchData {
    pub search: SearchResult,
}

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub issue_count: u64,
    pub page_info: PageInfo,
    pub nodes: Vec<PullRequest>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub has_next_page: bool,
    pub end_cursor: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_search_response() {
        let json = include_str!("../benchmarks/sample-response.json");
        let response: SearchResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.data.search.issue_count, 5);
        assert_eq!(response.data.search.nodes.len(), 5);
    }

    #[test]
    fn deserialize_pr_fields() {
        let json = include_str!("../benchmarks/sample-response.json");
        let response: SearchResponse = serde_json::from_str(json).unwrap();
        let first = &response.data.search.nodes[0];
        assert_eq!(first.title, "feat: add dark mode support");
        assert_eq!(first.number, 101);
        assert_eq!(first.repository.name_with_owner, "example/repo-a");
        assert_eq!(first.author.login, "alice");
    }

    #[test]
    fn deserialize_ci_states() {
        let json = include_str!("../benchmarks/sample-response.json");
        let response: SearchResponse = serde_json::from_str(json).unwrap();
        let nodes = &response.data.search.nodes;

        // 最後のPRは statusCheckRollup が null
        let last = nodes.last().unwrap();
        let rollup = last.commits.nodes[0].commit.status_check_rollup.as_ref();
        assert!(rollup.is_none());
    }

    #[test]
    fn deserialize_page_info() {
        let json = include_str!("../benchmarks/sample-response.json");
        let response: SearchResponse = serde_json::from_str(json).unwrap();
        assert!(!response.data.search.page_info.has_next_page);
        assert!(response.data.search.page_info.end_cursor.is_none());
    }
}
