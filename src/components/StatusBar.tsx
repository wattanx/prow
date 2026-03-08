import React from "react";
import { Box, Text } from "ink";
import type { AppMode, SortOrder } from "../types.js";

interface StatusBarProps {
  mode: AppMode;
  sortOrder: SortOrder;
  lastUpdated: Date | null;
  loading: boolean;
}

export function StatusBar({ mode, sortOrder, lastUpdated, loading }: StatusBarProps) {
  const timeStr = lastUpdated
    ? lastUpdated.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })
    : "-";

  return (
    <Box flexDirection="column" borderTop borderStyle="single" paddingX={1}>
      <Box justifyContent="space-between">
        <Text color="gray">
          {mode === "filter"
            ? "↑↓/jk move  Space toggle  ⏎ apply  Esc cancel"
            : "j/k move  h/l section  g/G top/end  ⏎ open  s sort  f filter  r refresh  q quit"}
        </Text>
        <Box gap={2}>
          {mode !== "filter" && (
            <Text color="gray">
              Sort: {sortOrder === "newest" ? "newest first" : "oldest first"}
            </Text>
          )}
          <Text color="gray">{loading ? "Loading..." : `Updated ${timeStr}`}</Text>
        </Box>
      </Box>
    </Box>
  );
}
