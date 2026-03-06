import { useState, useEffect, useCallback, useRef } from "react";
import type { PullRequest, TabType } from "../types.js";
import { createGitHubClient } from "../lib/github.js";
import { getGitHubToken } from "../lib/auth.js";

interface UsePRsResult {
  createdPRs: PullRequest[];
  reviewRequestedPRs: PullRequest[];
  loading: boolean;
  error: string | null;
  lastUpdated: Date | null;
  refresh: () => void;
}

export function usePRs(pollInterval: number): UsePRsResult {
  const [createdPRs, setCreatedPRs] = useState<PullRequest[]>([]);
  const [reviewRequestedPRs, setReviewRequestedPRs] = useState<PullRequest[]>(
    []
  );
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [lastUpdated, setLastUpdated] = useState<Date | null>(null);
  const clientRef = useRef<ReturnType<typeof createGitHubClient> | null>(null);

  const fetchData = useCallback(async () => {
    try {
      if (!clientRef.current) {
        const token = getGitHubToken();
        clientRef.current = createGitHubClient(token);
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

  return {
    createdPRs,
    reviewRequestedPRs,
    loading,
    error,
    lastUpdated,
    refresh: fetchData,
  };
}
