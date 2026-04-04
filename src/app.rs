use std::collections::{BTreeSet, HashMap};

use chrono::{DateTime, Utc};

use crate::{
    pr_classifier::{classify_new, classify_stale},
    types::{AppConfig, AppMode, PullRequest, SectionType, SortOrder},
};

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
        let prs = match self.active_section {
            SectionType::New => classify_new(&self.review_requested_prs),
            SectionType::Stale => classify_stale(&self.review_requested_prs),
            SectionType::All => self.review_requested_prs.clone(),
            SectionType::Authored => self.created_prs.clone(),
        };

        let filtered: Vec<PullRequest> = if self.is_filtering() {
            prs.into_iter()
                .filter(|pr| self.selected_repos.contains(&pr.repository.name_with_owner))
                .collect()
        } else {
            prs
        };

        let mut grouped: HashMap<String, Vec<PullRequest>> = HashMap::new();
        for pr in filtered {
            grouped
                .entry(pr.repository.name_with_owner.clone())
                .or_default()
                .push(pr);
        }

        let is_oldest = self.sort_order == SortOrder::Oldest;
        for prs in grouped.values_mut() {
            prs.sort_by(|a, b| {
                let cmp = b.updated_at.cmp(&a.updated_at);
                if is_oldest { cmp.reverse() } else { cmp }
            });
        }

        let mut sorted_groups: Vec<_> = grouped.into_iter().collect();
        sorted_groups.sort_by(|(_, a_prs), (_, b_prs)| {
            let a_time = &a_prs[0].updated_at;
            let b_time = &b_prs[0].updated_at;
            if is_oldest {
                a_time.cmp(b_time)
            } else {
                b_time.cmp(a_time)
            }
        });

        sorted_groups.into_iter().flat_map(|(_, prs)| prs).collect()
    }

    /// Recompute all_repos from created + review_requested PRs.
    /// See: src/hooks/useRepoFilter.ts — allRepos
    pub fn update_repos(&mut self) {
        let mut repos = BTreeSet::new();
        for pr in self
            .created_prs
            .iter()
            .chain(self.review_requested_prs.iter())
        {
            repos.insert(pr.repository.name_with_owner.clone());
        }
        self.all_repos = repos.into_iter().collect();
    }

    /// Switch to the next/previous section.
    /// See: src/app.tsx — switchSection()
    pub fn switch_section(&mut self, direction: i32) {
        let sections = SectionType::ALL_SECTIONS;
        let current = sections
            .iter()
            .position(|s| *s == self.active_section)
            .unwrap_or(0);
        let next = (current as i32 + direction).rem_euclid(sections.len() as i32) as usize;
        self.active_section = sections[next];
        self.selected_index = 0;
    }

    /// Toggle a repo in the filter.
    /// See: src/hooks/useRepoFilter.ts — toggleRepo()
    pub fn toggle_repo(&mut self, index: usize) {
        let repo = self.all_repos[index].clone();
        if self.selected_repos.is_empty() {
            self.selected_repos = BTreeSet::from_iter(self.all_repos.clone());
            self.selected_repos.remove(&repo);
        } else if self.selected_repos.contains(&repo) {
            self.selected_repos.remove(&repo);
        } else {
            self.selected_repos.insert(repo);
        }

        if self.selected_repos.len() == self.all_repos.len() {
            self.selected_repos.clear();
        }
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
        SectionCounts {
            new: classify_new(&self.review_requested_prs).len(),
            stale: classify_stale(&self.review_requested_prs).len(),
            all: self.review_requested_prs.len(),
            authored: self.created_prs.len(),
        }
    }
}
