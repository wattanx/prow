import React from "react";
import { Box, Text } from "ink";

interface SummaryBarProps {
  mine: number;
  authored: number;
  newCount: number;
  stale: number;
  lastUpdated: Date | null;
  loading: boolean;
}

export function SummaryBar({
  mine,
  authored,
  newCount,
  stale,
  lastUpdated,
  loading,
}: SummaryBarProps) {
  const timeStr = lastUpdated
    ? lastUpdated.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })
    : "-";

  return (
    <Box justifyContent="space-between" paddingX={1}>
      <Box gap={1}>
        <Text>
          Mine <Text bold>{mine}</Text>
        </Text>
        <Text color="gray">|</Text>
        <Text>
          Authored <Text bold>{authored}</Text>
        </Text>
        <Text color="gray">|</Text>
        <Text>
          New <Text bold>{newCount}</Text>
        </Text>
        <Text color="gray">|</Text>
        <Text>
          Stale <Text bold>{stale}</Text>
        </Text>
      </Box>
      <Text color="gray">{loading ? "Loading..." : `Updated ${timeStr}`}</Text>
    </Box>
  );
}
