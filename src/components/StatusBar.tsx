import React from "react";
import { Box, Text } from "ink";
import type { AppMode, SortOrder } from "../types.js";

interface StatusBarProps {
  mode: AppMode;
  sortOrder: SortOrder;
  filterRepos: string[];
}

export function StatusBar({ mode, sortOrder, filterRepos }: StatusBarProps) {
  return (
    <Box flexDirection="column" borderTop borderStyle="single" paddingX={1}>
      {filterRepos.length > 0 && mode !== "filter" && (
        <Text color="yellow">Filter: {filterRepos.join(", ")}</Text>
      )}
      <Box justifyContent="space-between">
        <Text color="gray">
          {mode === "filter"
            ? "↑↓/jk move  Space toggle  ⏎ apply  Esc cancel"
            : "j/k move  h/l section  g/G top/end  ⏎ open  s sort  f filter  r refresh  q quit"}
        </Text>
        {mode !== "filter" && (
          <Text color="gray">Sort: {sortOrder === "newest" ? "newest first" : "oldest first"}</Text>
        )}
      </Box>
    </Box>
  );
}
