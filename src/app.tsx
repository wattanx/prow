import React, { useState, useCallback, useMemo } from "react";
import { Box, Text, useApp, useInput } from "ink";
import type { SectionType, AppConfig, SortOrder } from "./types.js";
import { usePRs } from "./hooks/usePRs.js";
import { useRepoFilter } from "./hooks/useRepoFilter.js";
import { SummaryBar } from "./components/SummaryBar.js";
import { SectionList, SECTIONS } from "./components/SectionList.js";
import { PRList } from "./components/PRList.js";
import { RepoFilter } from "./components/RepoFilter.js";
import { StatusBar } from "./components/StatusBar.js";
import { openInBrowser } from "./lib/browser.js";
import type { AppMode } from "./types.js";

interface AppProps {
  config: AppConfig;
}

const EMPTY_MESSAGES: Record<SectionType, string> = {
  new: "No new review requests",
  stale: "No stale review requests",
  mine: "No review requests",
  authored: "No authored pull requests",
};

export function App({ config }: AppProps) {
  const { exit } = useApp();
  const [activeSection, setActiveSection] = useState<SectionType>(config.defaultSection);
  const [mode, setMode] = useState<AppMode>("list");
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [filterCursorIndex, setFilterCursorIndex] = useState(0);
  const [sortOrder, setSortOrder] = useState<SortOrder>("newest");

  const {
    createdPRs,
    reviewRequestedPRs,
    sectionPRs,
    sectionCounts,
    loading,
    error,
    lastUpdated,
    refresh,
  } = usePRs(config.pollInterval);

  const repoFilter = useRepoFilter(createdPRs, reviewRequestedPRs);

  const currentPRs = useMemo(() => {
    const prs = sectionPRs(activeSection);
    const filtered = repoFilter.isFiltering
      ? prs.filter((pr) => repoFilter.selectedRepos.has(pr.repository.nameWithOwner))
      : prs;

    const sorted = [...filtered];
    if (sortOrder === "oldest") {
      sorted.sort((a, b) => new Date(a.updatedAt).getTime() - new Date(b.updatedAt).getTime());
    } else {
      sorted.sort((a, b) => new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime());
    }
    return sorted;
  }, [activeSection, sectionPRs, repoFilter.isFiltering, repoFilter.selectedRepos, sortOrder]);

  const selectedPR = currentPRs[selectedIndex] ?? null;

  const switchSection = useCallback(() => {
    setActiveSection((prev) => {
      const idx = SECTIONS.indexOf(prev);
      return SECTIONS[(idx + 1) % SECTIONS.length]!;
    });
    setSelectedIndex(0);
  }, []);

  useInput((input, key) => {
    if (mode === "filter") {
      const maxIndex = repoFilter.allRepos.length;

      if (input === "j" || key.downArrow) {
        setFilterCursorIndex((prev) => Math.min(prev + 1, maxIndex));
      } else if (input === "k" || key.upArrow) {
        setFilterCursorIndex((prev) => Math.max(prev - 1, 0));
      } else if (input === " ") {
        if (filterCursorIndex === 0) {
          repoFilter.selectAll();
        } else {
          repoFilter.toggleRepo(repoFilter.allRepos[filterCursorIndex - 1]!);
        }
      } else if (key.return) {
        setMode("list");
        setSelectedIndex(0);
      } else if (key.escape) {
        setMode("list");
      }
      return;
    }

    // List mode
    if (input === "q") {
      exit();
    } else if (input === "j" || key.downArrow) {
      setSelectedIndex((prev) => Math.min(prev + 1, currentPRs.length - 1));
    } else if (input === "k" || key.upArrow) {
      setSelectedIndex((prev) => Math.max(prev - 1, 0));
    } else if (key.return) {
      if (selectedPR) {
        openInBrowser(selectedPR.url);
      }
    } else if (key.tab) {
      switchSection();
    } else if (input === "s") {
      setSortOrder((prev) => (prev === "newest" ? "oldest" : "newest"));
    } else if (input === "f") {
      if (repoFilter.allRepos.length > 0) {
        setMode("filter");
        setFilterCursorIndex(0);
      }
    } else if (input === "r") {
      refresh();
    }
  });

  if (error) {
    return (
      <Box flexDirection="column" padding={1}>
        <Text color="red" bold>
          Failed to fetch pull requests
        </Text>
        <Text color="gray">Press "r" to retry</Text>
      </Box>
    );
  }

  const filterRepoNames = repoFilter.isFiltering ? [...repoFilter.selectedRepos] : [];

  return (
    <Box flexDirection="column">
      <SummaryBar
        mine={sectionCounts.mine}
        authored={sectionCounts.authored}
        newCount={sectionCounts.new}
        stale={sectionCounts.stale}
        lastUpdated={lastUpdated}
        loading={loading}
      />

      <Box borderTop borderStyle="single">
        <SectionList activeSection={activeSection} counts={sectionCounts} />

        {mode === "filter" ? (
          <RepoFilter
            allRepos={repoFilter.allRepos}
            selectedRepos={repoFilter.selectedRepos}
            cursorIndex={filterCursorIndex}
          />
        ) : (
          <PRList
            prs={currentPRs}
            selectedIndex={selectedIndex}
            activeSection={activeSection}
            emptyMessage={EMPTY_MESSAGES[activeSection]}
          />
        )}
      </Box>

      <StatusBar mode={mode} sortOrder={sortOrder} filterRepos={filterRepoNames} />
    </Box>
  );
}
