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
  reviewRequestedPRs: PullRequest[],
  initialRepos: string[] = [],
): UseRepoFilterResult {
  const allRepos = useMemo(() => {
    const repos = new Set<string>();
    for (const pr of [...createdPRs, ...reviewRequestedPRs]) {
      repos.add(pr.repository.nameWithOwner);
    }
    return [...repos].sort();
  }, [createdPRs, reviewRequestedPRs]);

  const [selectedRepos, setSelectedRepos] = useState<Set<string>>(() => new Set(initialRepos));

  const isFiltering = selectedRepos.size > 0 && selectedRepos.size < allRepos.length;

  const toggleRepo = useCallback(
    (repo: string) => {
      setSelectedRepos((prev) => {
        // When "All" is active (empty set), expand to all repos then deselect the toggled one
        if (prev.size === 0) {
          const next = new Set(allRepos);
          next.delete(repo);
          return next;
        }
        const next = new Set(prev);
        if (next.has(repo)) {
          next.delete(repo);
        } else {
          next.add(repo);
        }
        // If all repos are selected again, reset to empty (= All)
        if (next.size === allRepos.length) {
          return new Set<string>();
        }
        return next;
      });
    },
    [allRepos],
  );

  const selectAll = useCallback(() => {
    setSelectedRepos(new Set());
  }, []);

  const clearAll = useCallback(() => {
    setSelectedRepos(new Set());
  }, []);

  const filterPRs = useCallback(
    (prs: PullRequest[]): PullRequest[] => {
      if (!isFiltering) return prs;
      return prs.filter((pr) => selectedRepos.has(pr.repository.nameWithOwner));
    },
    [isFiltering, selectedRepos],
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
