use anyhow::Result;

use crate::types::PullRequest;

/// GraphQL fragment for PR fields.
/// See: src/lib/github.ts — PR_FRAGMENT
const PR_FRAGMENT: &str = r#"
  title
  url
  number
  state
  isDraft
  createdAt
  updatedAt
  repository {
    nameWithOwner
  }
  author {
    login
  }
  labels(first: 10) {
    nodes {
      name
      color
    }
  }
  reviewDecision
  reviewRequests {
    totalCount
  }
  reviews(first: 1) {
    totalCount
  }
  commits(last: 1) {
    nodes {
      commit {
        statusCheckRollup {
          state
        }
      }
    }
  }
"#;

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
          PR_FRAGMENT
        }
      }
    }
  }
"#;

/// Trait for GitHub API access (allows mocking in tests).
pub trait GitHubClient {
    fn fetch_created_prs(&self) -> impl std::future::Future<Output = Result<Vec<PullRequest>>> + Send;
    fn fetch_review_requested_prs(&self) -> impl std::future::Future<Output = Result<Vec<PullRequest>>> + Send;
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
        todo!("Shell out to `gh api graphql` and fetch created PRs with pagination")
    }

    /// Fetch PRs where review is requested from the current user.
    /// Query: "review-requested:@me is:pr is:open sort:updated-desc"
    ///
    /// See: src/lib/github.ts — fetchReviewRequestedPRs()
    async fn fetch_review_requested_prs(&self) -> Result<Vec<PullRequest>> {
        todo!("Shell out to `gh api graphql` and fetch review-requested PRs with pagination")
    }
}

/// Execute a GraphQL query via `gh api graphql` and parse the response.
///
/// See: src/lib/github.ts — ghGraphql()
async fn gh_graphql(_query: &str, _search_query: &str, _first: u32, _after: Option<&str>) -> Result<SearchResponse> {
    todo!("Execute gh api graphql subprocess and parse JSON response")
}

/// Fetch all pages of a search query.
///
/// See: src/lib/github.ts — fetchAllPages()
async fn fetch_all_pages(_search_query: &str) -> Result<Vec<PullRequest>> {
    todo!("Paginate through all results using gh_graphql()")
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
    // TODO: Test JSON deserialization with sample gh output
    // TODO: Test pagination logic
}
