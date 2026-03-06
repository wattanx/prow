export interface PullRequest {
  title: string;
  url: string;
  number: number;
  state: "OPEN" | "CLOSED" | "MERGED";
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

export type TabType = "created" | "reviewRequested";

export type ColumnKey = "repo" | "title" | "ci" | "reviews" | "labels" | "updatedAt";

export interface AppConfig {
  columns: ColumnKey[];
  pollInterval: number;
  defaultTab: TabType;
}

export type AppMode = "list" | "filter";
