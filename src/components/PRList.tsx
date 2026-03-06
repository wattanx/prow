import React from "react";
import { Box, Text } from "ink";
import type { PullRequest, ColumnKey } from "../types.js";
import { PRRow } from "./PRRow.js";

interface PRListProps {
  prs: PullRequest[];
  selectedIndex: number;
  columns: ColumnKey[];
}

const COLUMN_HEADERS: Record<ColumnKey, { label: string; width?: number }> = {
  repo: { label: "Repo", width: 24 },
  title: { label: "Title" },
  ci: { label: "CI", width: 3 },
  reviews: { label: "Review", width: 12 },
  labels: { label: "Labels", width: 16 },
  updatedAt: { label: "Age", width: 5 },
};

export function PRList({ prs, selectedIndex, columns }: PRListProps) {
  if (prs.length === 0) {
    return (
      <Box paddingLeft={2} paddingTop={1}>
        <Text color="gray">No pull requests found.</Text>
      </Box>
    );
  }

  return (
    <Box flexDirection="column">
      <Box gap={1} paddingLeft={3}>
        {columns.map((col) => {
          const header = COLUMN_HEADERS[col];
          return (
            <Box key={col} width={header.width} flexGrow={header.width ? undefined : 1}>
              <Text bold color="gray">
                {header.label}
              </Text>
            </Box>
          );
        })}
      </Box>
      {prs.map((pr, index) => (
        <PRRow
          key={`${pr.repository.nameWithOwner}#${pr.number}`}
          pr={pr}
          isSelected={index === selectedIndex}
          columns={columns}
        />
      ))}
    </Box>
  );
}
