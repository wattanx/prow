import React from "react";
import { Box, Text } from "ink";
import type { AppMode } from "../types.js";

interface StatusBarProps {
  mode: AppMode;
  lastUpdated: Date | null;
  filterRepos: string[];
  loading: boolean;
}

export function StatusBar({
  mode,
  lastUpdated,
  filterRepos,
  loading,
}: StatusBarProps) {
  const timeStr = lastUpdated
    ? lastUpdated.toLocaleTimeString()
    : "-";

  return (
    <Box flexDirection="column" borderTop borderStyle="single" paddingX={1}>
      {filterRepos.length > 0 && mode !== "filter" && (
        <Text color="yellow">Filter: {filterRepos.join(", ")}</Text>
      )}
      <Box justifyContent="space-between">
        <Text color="gray">
          {mode === "filter"
            ? "↑↓/jk move  Space toggle  ⏎ apply  Esc cancel"
            : "↑↓/jk move  ⏎ open  Tab switch  f filter  r refresh  q quit"}
        </Text>
        <Text color="gray">
          {loading ? "Loading..." : `Updated: ${timeStr}`}
        </Text>
      </Box>
    </Box>
  );
}
