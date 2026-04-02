import React from "react";
import { Box, Text } from "ink";
import type { PullRequest } from "../types.js";

interface PRRowProps {
  pr: PullRequest;
  isSelected: boolean;
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

function getCIStatus(pr: PullRequest): { icon: string; color: string } {
  const state = pr.commits.nodes[0]?.commit.statusCheckRollup?.state ?? null;
  switch (state) {
    case "SUCCESS":
      return { icon: "✓", color: "green" };
    case "FAILURE":
    case "ERROR":
      return { icon: "✗", color: "red" };
    case "PENDING":
    case "EXPECTED":
      return { icon: "◌", color: "yellow" };
    default:
      return { icon: "-", color: "gray" };
  }
}

export function PRRow({ pr, isSelected }: PRRowProps) {
  const bg = isSelected ? "gray" : undefined;
  const ci = getCIStatus(pr);

  return (
    <Box paddingX={1} backgroundColor={bg}>
      <Text>{isSelected ? "> " : "  "}</Text>
      <Text color={ci.color}>{ci.icon} </Text>
      <Box flexGrow={1}>
        <Text wrap="truncate">{pr.title}</Text>
      </Box>
      <Box width={5} justifyContent="flex-end">
        <Text color={isSelected ? undefined : "gray"}>{formatRelativeTime(pr.updatedAt)}</Text>
      </Box>
      <Box width={16} justifyContent="flex-end">
        <Text color={isSelected ? undefined : "gray"}>{pr.author.login}</Text>
      </Box>
    </Box>
  );
}
