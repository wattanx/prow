import type { PullRequest } from "../types.js";

let counter = 0;

export function createMockPR(
  overrides: Partial<PullRequest> & { title?: string } = {},
): PullRequest {
  counter++;
  return {
    title: `Test PR ${counter}`,
    url: `https://github.com/owner/repo/pull/${counter}`,
    number: counter,
    state: "OPEN",
    isDraft: false,
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString(),
    repository: {
      nameWithOwner: "owner/repo",
    },
    author: {
      login: "testuser",
    },
    labels: {
      nodes: [],
    },
    reviewDecision: null,
    reviewRequests: {
      totalCount: 0,
    },
    reviews: {
      totalCount: 0,
    },
    commits: {
      nodes: [
        {
          commit: {
            statusCheckRollup: null,
          },
        },
      ],
    },
    ...overrides,
  };
}
