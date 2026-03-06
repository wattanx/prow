import React from "react";
import { Box, Text } from "ink";
import type { PullRequest, ColumnKey } from "../types.js";

interface PRRowProps {
  pr: PullRequest;
  isSelected: boolean;
  columns: ColumnKey[];
}

function getCIStatus(pr: PullRequest): { symbol: string; color: string } {
  const rollup = pr.commits.nodes[0]?.commit.statusCheckRollup;
  if (!rollup) return { symbol: "-", color: "gray" };

  switch (rollup.state) {
    case "SUCCESS":
      return { symbol: "✓", color: "green" };
    case "FAILURE":
    case "ERROR":
      return { symbol: "✗", color: "red" };
    case "PENDING":
    case "EXPECTED":
      return { symbol: "○", color: "yellow" };
    default:
      return { symbol: "-", color: "gray" };
  }
}

function getReviewStatus(pr: PullRequest): {
  text: string;
  color: string;
} {
  switch (pr.reviewDecision) {
    case "APPROVED":
      return { text: "✓ Approved", color: "green" };
    case "CHANGES_REQUESTED":
      return { text: "✗ Changes", color: "red" };
    case "REVIEW_REQUIRED":
      return { text: "● Review", color: "yellow" };
    default:
      return { text: "-", color: "gray" };
  }
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

export function PRRow({ pr, isSelected, columns }: PRRowProps) {
  const ci = getCIStatus(pr);
  const review = getReviewStatus(pr);

  const columnRenderers: Record<ColumnKey, React.ReactNode> = {
    repo: (
      <Box key="repo" width={24}>
        <Text color="cyan" wrap="truncate">
          {pr.repository.nameWithOwner}
        </Text>
      </Box>
    ),
    title: (
      <Box key="title" flexGrow={1}>
        <Text wrap="truncate">{pr.title}</Text>
      </Box>
    ),
    ci: (
      <Box key="ci" width={3}>
        <Text color={ci.color}>{ci.symbol}</Text>
      </Box>
    ),
    reviews: (
      <Box key="reviews" width={12}>
        <Text color={review.color}>{review.text}</Text>
      </Box>
    ),
    labels: (
      <Box key="labels" width={16}>
        <Text wrap="truncate" color="magenta">
          {pr.labels.nodes.map((l) => l.name).join(", ") || "-"}
        </Text>
      </Box>
    ),
    updatedAt: (
      <Box key="updatedAt" width={5}>
        <Text color="gray">{formatRelativeTime(pr.updatedAt)}</Text>
      </Box>
    ),
  };

  return (
    <Box>
      <Text color={isSelected ? "blue" : undefined}>
        {isSelected ? ">" : " "}{" "}
      </Text>
      <Box gap={1}>
        {columns.map((col) => columnRenderers[col])}
      </Box>
    </Box>
  );
}
