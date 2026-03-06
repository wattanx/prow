import React, { useState, useCallback } from "react";
import { Box, Text, useApp, useInput } from "ink";
import type { TabType, AppMode, AppConfig } from "./types.js";
import { usePRs } from "./hooks/usePRs.js";
import { useRepoFilter } from "./hooks/useRepoFilter.js";
import { TabBar } from "./components/TabBar.js";
import { PRList } from "./components/PRList.js";
import { RepoFilter } from "./components/RepoFilter.js";
import { StatusBar } from "./components/StatusBar.js";
import { openInBrowser } from "./lib/browser.js";

interface AppProps {
  config: AppConfig;
}

export function App({ config }: AppProps) {
  const { exit } = useApp();
  const [activeTab, setActiveTab] = useState<TabType>(config.defaultTab);
  const [mode, setMode] = useState<AppMode>("list");
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [filterCursorIndex, setFilterCursorIndex] = useState(0);

  const { createdPRs, reviewRequestedPRs, loading, error, lastUpdated, refresh } = usePRs(
    config.pollInterval,
  );

  const repoFilter = useRepoFilter(createdPRs, reviewRequestedPRs);

  const currentPRs =
    activeTab === "created"
      ? repoFilter.filterPRs(createdPRs)
      : repoFilter.filterPRs(reviewRequestedPRs);

  const filteredCreatedCount = repoFilter.filterPRs(createdPRs).length;
  const filteredReviewCount = repoFilter.filterPRs(reviewRequestedPRs).length;

  const switchTab = useCallback(() => {
    setActiveTab((prev) => (prev === "created" ? "reviewRequested" : "created"));
    setSelectedIndex(0);
  }, []);

  useInput((input, key) => {
    if (mode === "filter") {
      const maxIndex = repoFilter.allRepos.length; // 0 = All, 1..n = repos

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
      const pr = currentPRs[selectedIndex];
      if (pr) {
        openInBrowser(pr.url);
      }
    } else if (key.tab) {
      switchTab();
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
          Error: {error}
        </Text>
      </Box>
    );
  }

  const filterRepoNames = repoFilter.isFiltering ? [...repoFilter.selectedRepos] : [];

  return (
    <Box flexDirection="column">
      <TabBar
        activeTab={activeTab}
        createdCount={filteredCreatedCount}
        reviewRequestedCount={filteredReviewCount}
      />

      {mode === "filter" ? (
        <RepoFilter
          allRepos={repoFilter.allRepos}
          selectedRepos={repoFilter.selectedRepos}
          cursorIndex={filterCursorIndex}
        />
      ) : (
        <PRList prs={currentPRs} selectedIndex={selectedIndex} columns={config.columns} />
      )}

      <StatusBar
        mode={mode}
        lastUpdated={lastUpdated}
        filterRepos={filterRepoNames}
        loading={loading}
      />
    </Box>
  );
}
