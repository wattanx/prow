import { useState, useMemo, useCallback } from "react";
import type { PullRequest } from "../types.js";

interface UseRepoFilterResult {
  selectedRepos: Set<string>;
  allRepos: string[];
  toggleRepo: (repo: string) => void;
  selectAll: () => void;
  clearAll: () => void;
  isFiltering: boolean;
  filterPRs: (prs: PullRequest[]) => PullRequest[];
}

export function useRepoFilter(
  createdPRs: PullRequest[],
  reviewRequestedPRs: PullRequest[]
): UseRepoFilterResult {
  const allRepos = useMemo(() => {
    const repos = new Set<string>();
    for (const pr of [...createdPRs, ...reviewRequestedPRs]) {
      repos.add(pr.repository.nameWithOwner);
    }
    return [...repos].sort();
  }, [createdPRs, reviewRequestedPRs]);

  const [selectedRepos, setSelectedRepos] = useState<Set<string>>(new Set());

  const isFiltering = selectedRepos.size > 0 && selectedRepos.size < allRepos.length;

  const toggleRepo = useCallback((repo: string) => {
    setSelectedRepos((prev) => {
      const next = new Set(prev);
      if (next.has(repo)) {
        next.delete(repo);
      } else {
        next.add(repo);
      }
      return next;
    });
  }, []);

  const selectAll = useCallback(() => {
    setSelectedRepos(new Set());
  }, []);

  const clearAll = useCallback(() => {
    setSelectedRepos(new Set());
  }, []);

  const filterPRs = useCallback(
    (prs: PullRequest[]): PullRequest[] => {
      if (!isFiltering) return prs;
      return prs.filter((pr) =>
        selectedRepos.has(pr.repository.nameWithOwner)
      );
    },
    [isFiltering, selectedRepos]
  );

  return {
    selectedRepos,
    allRepos,
    toggleRepo,
    selectAll,
    clearAll,
    isFiltering,
    filterPRs,
  };
}
