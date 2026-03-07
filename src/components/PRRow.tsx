import React from "react";
import { Box, Text } from "ink";
import type { PullRequest, PRKind } from "../types.js";

interface PRRowProps {
  pr: PullRequest;
  isSelected: boolean;
  kind: PRKind;
}

function formatRelativeTime(dateStr: string): string {
  const now = Date.now();
  const date = new Date(dateStr).getTime();
  const diff = now - date;

  const minutes = Math.floor(diff / 60000);
  if (minutes < 60) return `${minutes}m`;

  const hours = Math.floor(minutes / 60);
  if (hours < 24) return `${hours}h`;

  const days = Math.floor(hours / 24);
  if (days < 30) return `${days}d`;

  const months = Math.floor(days / 30);
  return `${months}mo`;
}

export function PRRow({ pr, isSelected, kind }: PRRowProps) {
  return (
    <Box>
      <Text color={isSelected ? "blue" : undefined}>{isSelected ? ">" : " "} </Text>
      <Box flexGrow={1}>
        <Text wrap="truncate">{pr.title}</Text>
      </Box>
      <Box width={5} justifyContent="flex-end">
        <Text color="gray">{formatRelativeTime(pr.updatedAt)}</Text>
      </Box>
      <Box width={16} justifyContent="flex-end">
        <Text color={kind === "authored draft" ? "yellow" : "gray"}>{kind}</Text>
      </Box>
    </Box>
  );
}
