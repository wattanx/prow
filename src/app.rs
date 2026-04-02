use std::collections::{BTreeSet, HashMap};

use chrono::{DateTime, Utc};

use crate::types::{AppConfig, AppMode, PullRequest, SectionType, SortOrder};

/// Central application state.
/// Replaces all React useState hooks from src/app.tsx.
pub struct AppState {
    pub mode: AppMode,
    pub active_section: SectionType,
    pub selected_index: usize,
    pub filter_cursor_index: usize,
    pub sort_order: SortOrder,

    // PR data
    pub created_prs: Vec<PullRequest>,
    pub review_requested_prs: Vec<PullRequest>,
    pub loading: bool,
    pub error: Option<String>,
    pub last_updated: Option<DateTime<Utc>>,

    // Repo filter state (from src/hooks/useRepoFilter.ts)
    pub all_repos: Vec<String>,
    pub selected_repos: BTreeSet<String>,

    // Config
    pub config: AppConfig,

    pub should_quit: bool,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        let selected_repos = BTreeSet::from_iter(config.filtered_repos.clone());
        Self {
            mode: AppMode::List,
            active_section: config.default_section,
            selected_index: 0,
            filter_cursor_index: 0,
            sort_order: SortOrder::Newest,
            created_prs: Vec::new(),
            review_requested_prs: Vec::new(),
            loading: true,
            error: None,
            last_updated: None,
            all_repos: Vec::new(),
            selected_repos,
            config,
            should_quit: false,
        }
    }

    /// Whether repo filtering is active (some repos selected, but not all).
    /// See: src/hooks/useRepoFilter.ts — isFiltering
    pub fn is_filtering(&self) -> bool {
        !self.selected_repos.is_empty() && self.selected_repos.len() < self.all_repos.len()
    }

    /// Get PRs for the active section, filtered and sorted.
    /// See: src/app.tsx — currentPRs (lines 46-79)
    pub fn current_prs(&self) -> Vec<PullRequest> {
        todo!("Classify PRs into sections, apply repo filter, group by repo, sort")
    }

    /// Recompute all_repos from created + review_requested PRs.
    /// See: src/hooks/useRepoFilter.ts — allRepos
    pub fn update_repos(&mut self) {
        todo!("Derive sorted unique repo list from all PRs")
    }

    /// Switch to the next/previous section.
    /// See: src/app.tsx — switchSection()
    pub fn switch_section(&mut self, direction: i32) {
        todo!("Cycle through sections, reset selected_index")
    }

    /// Toggle a repo in the filter.
    /// See: src/hooks/useRepoFilter.ts — toggleRepo()
    pub fn toggle_repo(&mut self, index: usize) {
        todo!("Toggle repo selection, handle 'All' logic")
    }

    /// Select all repos (reset filter).
    /// See: src/hooks/useRepoFilter.ts — selectAll()
    pub fn select_all_repos(&mut self) {
        self.selected_repos.clear();
    }
}

/// Section-specific empty messages.
/// See: src/app.tsx — EMPTY_MESSAGES
pub fn empty_message(section: SectionType) -> &'static str {
    match section {
        SectionType::New => "No new review requests",
        SectionType::Stale => "No stale review requests",
        SectionType::All => "No review requests",
        SectionType::Authored => "No authored pull requests",
    }
}

/// Section counts.
pub struct SectionCounts {
    pub new: usize,
    pub stale: usize,
    pub all: usize,
    pub authored: usize,
}

impl SectionCounts {
    pub fn get(&self, section: SectionType) -> usize {
        match section {
            SectionType::New => self.new,
            SectionType::Stale => self.stale,
            SectionType::All => self.all,
            SectionType::Authored => self.authored,
        }
    }
}

impl AppState {
    /// Compute section counts.
    /// See: src/hooks/usePRs.ts — sectionCounts
    pub fn section_counts(&self) -> SectionCounts {
        todo!("Count PRs per section using classify_new/classify_stale")
    }
}
