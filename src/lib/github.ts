import { graphql } from "@octokit/graphql";
import type { PullRequest } from "../types.js";

const PR_FRAGMENT = `
  title
  url
  number
  state
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
  search: {
    issueCount: number;
    pageInfo: {
      hasNextPage: boolean;
      endCursor: string | null;
    };
    nodes: PullRequest[];
  };
}

export function createGitHubClient(token: string) {
  const ghGraphql = graphql.defaults({
    headers: {
      authorization: `token ${token}`,
    },
  });

  async function fetchAllPages(searchQuery: string): Promise<PullRequest[]> {
    const allPRs: PullRequest[] = [];
    let after: string | null = null;

    do {
      const response = await ghGraphql<SearchResponse>(SEARCH_QUERY, {
        searchQuery,
        first: 50,
        after,
      });

      allPRs.push(...response.search.nodes);

      if (response.search.pageInfo.hasNextPage) {
        after = response.search.pageInfo.endCursor;
      } else {
        break;
      }
    } while (true);

    return allPRs;
  }

  return {
    async fetchCreatedPRs(): Promise<PullRequest[]> {
      return fetchAllPages("author:@me is:pr is:open sort:updated-desc");
    },

    async fetchReviewRequestedPRs(): Promise<PullRequest[]> {
      return fetchAllPages("review-requested:@me is:pr is:open sort:updated-desc");
    },
  };
}
