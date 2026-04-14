use serde::Deserialize;

// -- PR state from GitHub GraphQL API --

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrState {
    Open,
    Closed,
    Merged,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReviewDecision {
    Approved,
    ChangesRequested,
    ReviewRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CheckState {
    Success,
    Failure,
    Pending,
    Error,
    Expected,
}

// -- GitHub GraphQL response structs --
// See: src/types.ts

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PullRequest {
    pub title: String,
    pub url: String,
    pub number: u64,
    pub state: PrState,
    pub is_draft: bool,
    pub created_at: String,
    pub updated_at: String,
    pub repository: Repository,
    pub author: Author,
    pub labels: Labels,
    pub review_decision: Option<ReviewDecision>,
    pub review_requests: CountNode,
    pub reviews: CountNode,
    pub commits: CommitNodes,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub name_with_owner: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Author {
    pub login: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Labels {
    pub nodes: Vec<Label>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Label {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountNode {
    pub total_count: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommitNodes {
    pub nodes: Vec<CommitNode>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommitNode {
    pub commit: CommitInfo,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitInfo {
    pub status_check_rollup: Option<StatusCheckRollup>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StatusCheckRollup {
    pub state: Option<CheckState>,
}

// -- App types --

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionType {
    New,
    Stale,
    All,
    Authored,
}

impl SectionType {
    pub const ALL_SECTIONS: [SectionType; 4] = [
        SectionType::New,
        SectionType::Stale,
        SectionType::All,
        SectionType::Authored,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            SectionType::New => "new",
            SectionType::Stale => "stale",
            SectionType::All => "all",
            SectionType::Authored => "authored",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Newest,
    Oldest,
}

impl SortOrder {
    pub fn toggle(&self) -> Self {
        match self {
            SortOrder::Newest => SortOrder::Oldest,
            SortOrder::Oldest => SortOrder::Newest,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    List,
    Filter,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub poll_interval: u64,
    pub default_section: SectionType,
    pub filtered_repos: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            poll_interval: 60,
            default_section: SectionType::All,
            filtered_repos: Vec::new(),
        }
    }
}
