import { execFile } from "node:child_process";
import { promisify } from "node:util";
import type { PullRequest } from "../types.js";

const execFileAsync = promisify(execFile);

const PR_FRAGMENT = `
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
`;

const SEARCH_QUERY = `
  query($searchQuery: String!, $first: Int!, $after: String) {
    search(query: $searchQuery, type: ISSUE, first: $first, after: $after) {
      issueCount
      pageInfo {
        hasNextPage
        endCursor
      }
      nodes {
        ... on PullRequest {
          ${PR_FRAGMENT}
        }
      }
    }
  }
`;

interface SearchResponse {
  data: {
    search: {
      issueCount: number;
      pageInfo: {
        hasNextPage: boolean;
        endCursor: string | null;
      };
      nodes: PullRequest[];
    };
  };
}

async function ghGraphql(
  query: string,
  variables: Record<string, unknown>,
): Promise<SearchResponse> {
  const args = ["api", "graphql", "-f", `query=${query}`];

  for (const [key, value] of Object.entries(variables)) {
    if (value === null || value === undefined) continue;
    if (typeof value === "number") {
      args.push("-F", `${key}=${value}`);
    } else {
      args.push("-f", `${key}=${value}`);
    }
  }

  const { stdout } = await execFileAsync("gh", args);
  return JSON.parse(stdout);
}

async function fetchAllPages(searchQuery: string): Promise<PullRequest[]> {
  const allPRs: PullRequest[] = [];
  let after: string | null = null;

  let hasNextPage = true;

  while (hasNextPage) {
    const response = await ghGraphql(SEARCH_QUERY, {
      searchQuery,
      first: 50,
      after,
    });

    allPRs.push(...response.data.search.nodes);

    hasNextPage = response.data.search.pageInfo.hasNextPage;
    after = response.data.search.pageInfo.endCursor;
  }

  return allPRs;
}

export function createGitHubClient() {
  return {
    async fetchCreatedPRs(): Promise<PullRequest[]> {
      return fetchAllPages("author:@me is:pr is:open sort:updated-desc");
    },

    async fetchReviewRequestedPRs(): Promise<PullRequest[]> {
      return fetchAllPages("review-requested:@me is:pr is:open sort:updated-desc");
    },
  };
}
