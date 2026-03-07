import { useState, useEffect, useCallback, useMemo, useRef } from "react";
import type { PullRequest, SectionType } from "../types.js";
import { createGitHubClient } from "../lib/github.js";

const STALE_THRESHOLD_MS = 48 * 60 * 60 * 1000; // 48 hours

interface SectionCounts {
  new: number;
  stale: number;
  mine: number;
  authored: number;
}

interface UsePRsResult {
  createdPRs: PullRequest[];
  reviewRequestedPRs: PullRequest[];
  sectionPRs: (section: SectionType) => PullRequest[];
  sectionCounts: SectionCounts;
  loading: boolean;
  error: string | null;
  lastUpdated: Date | null;
  refresh: () => void;
}

function classifyNew(prs: PullRequest[]): PullRequest[] {
  return prs.filter((pr) => {
    const hasNoReviews = pr.reviews.totalCount === 0;
    const age = Date.now() - new Date(pr.updatedAt).getTime();
    const isRecent = age < STALE_THRESHOLD_MS;
    return hasNoReviews && isRecent;
  });
}

function classifyStale(prs: PullRequest[]): PullRequest[] {
  return prs.filter((pr) => {
    const age = Date.now() - new Date(pr.updatedAt).getTime();
    return age >= STALE_THRESHOLD_MS;
  });
}

export function usePRs(pollInterval: number): UsePRsResult {
  const [createdPRs, setCreatedPRs] = useState<PullRequest[]>([]);
  const [reviewRequestedPRs, setReviewRequestedPRs] = useState<PullRequest[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [lastUpdated, setLastUpdated] = useState<Date | null>(null);
  const clientRef = useRef<ReturnType<typeof createGitHubClient> | null>(null);

  const fetchData = useCallback(async () => {
    try {
      if (!clientRef.current) {
        clientRef.current = createGitHubClient();
      }

      setLoading(true);
      setError(null);

      const [created, reviewRequested] = await Promise.all([
        clientRef.current.fetchCreatedPRs(),
        clientRef.current.fetchReviewRequestedPRs(),
      ]);

      setCreatedPRs(created);
      setReviewRequestedPRs(reviewRequested);
      setLastUpdated(new Date());
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchData();
  }, [fetchData]);

  useEffect(() => {
    if (pollInterval <= 0) return;

    const interval = setInterval(fetchData, pollInterval * 1000);
    return () => clearInterval(interval);
  }, [fetchData, pollInterval]);

  const newPRs = useMemo(() => classifyNew(reviewRequestedPRs), [reviewRequestedPRs]);
  const stalePRs = useMemo(() => classifyStale(reviewRequestedPRs), [reviewRequestedPRs]);

  const sectionCounts: SectionCounts = useMemo(
    () => ({
      new: newPRs.length,
      stale: stalePRs.length,
      mine: reviewRequestedPRs.length,
      authored: createdPRs.length,
    }),
    [newPRs, stalePRs, reviewRequestedPRs, createdPRs],
  );

  const sectionPRs = useCallback(
    (section: SectionType): PullRequest[] => {
      switch (section) {
        case "new":
          return newPRs;
        case "stale":
          return stalePRs;
        case "mine":
          return reviewRequestedPRs;
        case "authored":
          return createdPRs;
      }
    },
    [newPRs, stalePRs, reviewRequestedPRs, createdPRs],
  );

  return {
    createdPRs,
    reviewRequestedPRs,
    sectionPRs,
    sectionCounts,
    loading,
    error,
    lastUpdated,
    refresh: fetchData,
  };
}
