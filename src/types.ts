export interface PullRequest {
  title: string;
  url: string;
  number: number;
  state: "OPEN" | "CLOSED" | "MERGED";
  isDraft: boolean;
  createdAt: string;
  updatedAt: string;
  repository: {
    nameWithOwner: string;
  };
  author: {
    login: string;
  };
  labels: {
    nodes: Array<{
      name: string;
      color: string;
    }>;
  };
  reviewDecision: "APPROVED" | "CHANGES_REQUESTED" | "REVIEW_REQUIRED" | null;
  reviewRequests: {
    totalCount: number;
  };
  reviews: {
    totalCount: number;
  };
  commits: {
    nodes: Array<{
      commit: {
        statusCheckRollup: {
          state: "SUCCESS" | "FAILURE" | "PENDING" | "ERROR" | "EXPECTED" | null;
        } | null;
      };
    }>;
  };
}

export type SectionType = "new" | "stale" | "mine" | "authored";

export type SortOrder = "oldest" | "newest";

export type PRKind = "mine" | "authored" | "authored draft";

export interface AppConfig {
  pollInterval: number;
  defaultSection: SectionType;
  filteredRepos: string[];
}

export type AppMode = "list" | "filter";
