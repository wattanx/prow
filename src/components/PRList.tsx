import React, { useMemo } from "react";
import { Box, Text, useStdout } from "ink";
import type { PullRequest } from "../types.js";
import { PRRow } from "./PRRow.js";

interface PRListProps {
  prs: PullRequest[];
  selectedIndex: number;
  emptyMessage: string;
}

interface DisplayRow {
  type: "repo-header" | "pr" | "spacer";
  repo?: string;
  pr?: PullRequest;
  flatIndex?: number;
}

function buildDisplayRows(prs: PullRequest[]): DisplayRow[] {
  const rows: DisplayRow[] = [];
  let currentRepo = "";
  let flatIndex = 0;

  for (const pr of prs) {
    const repo = pr.repository.nameWithOwner;
    if (repo !== currentRepo) {
      if (currentRepo !== "") {
        rows.push({ type: "spacer" });
      }
      currentRepo = repo;
      rows.push({ type: "repo-header", repo });
    }
    rows.push({ type: "pr", pr, flatIndex: flatIndex++ });
  }

  return rows;
}

function getViewport(
  rows: DisplayRow[],
  selectedIndex: number,
  maxVisible: number,
): { start: number; end: number } {
  const selectedRowIdx = rows.findIndex(
    (row) => row.type === "pr" && row.flatIndex === selectedIndex,
  );

  if (selectedRowIdx === -1 || rows.length <= maxVisible) {
    return { start: 0, end: rows.length };
  }

  // Reserve 2 lines for "..." indicators (worst case: top + bottom)
  const contentMax = maxVisible - 2;
  const half = Math.floor(contentMax / 2);
  let start = Math.max(0, selectedRowIdx - half);
  let end = start + contentMax;

  if (end > rows.length) {
    end = rows.length;
    start = Math.max(0, end - contentMax);
  }

  // Reclaim unused indicator slots
  if (start === 0) {
    end = Math.min(rows.length, end + 1);
  }
  if (end === rows.length) {
    start = Math.max(0, start - 1);
  }

  return { start, end };
}

export function PRList({ prs, selectedIndex, emptyMessage }: PRListProps) {
  const { stdout } = useStdout();
  // Reserve lines for SummaryBar(1) + border(1) + StatusBar(2) + padding
  const maxVisible = (stdout?.rows ?? 24) - 6;

  const displayRows = useMemo(() => buildDisplayRows(prs), [prs]);
  const { start, end } = useMemo(
    () => getViewport(displayRows, selectedIndex, maxVisible),
    [displayRows, selectedIndex, maxVisible],
  );

  if (prs.length === 0) {
    return (
      <Box flexGrow={1} height={maxVisible} paddingLeft={2} paddingTop={1}>
        <Text color="gray">{emptyMessage}</Text>
      </Box>
    );
  }

  const visibleRows = displayRows.slice(start, end);
  const paddingRows = Math.max(
    0,
    maxVisible - visibleRows.length - (start > 0 ? 1 : 0) - (end < displayRows.length ? 1 : 0),
  );

  return (
    <Box flexDirection="column" flexGrow={1} height={maxVisible} paddingLeft={2}>
      {start > 0 && (
        <Box>
          <Text color="gray">...</Text>
        </Box>
      )}
      {visibleRows.map((row, i) => {
        if (row.type === "repo-header") {
          return (
            <Box key={`header-${row.repo}-${start + i}`}>
              <Text bold color="cyan">
                {row.repo}
              </Text>
            </Box>
          );
        }
        if (row.type === "spacer") {
          return <Box key={`spacer-${start + i}`} height={1} />;
        }
        return (
          <PRRow
            key={`pr-${row.pr!.repository.nameWithOwner}#${row.pr!.number}`}
            pr={row.pr!}
            isSelected={row.flatIndex === selectedIndex}
          />
        );
      })}
      {end < displayRows.length && (
        <Box>
          <Text color="gray">...</Text>
        </Box>
      )}
      {paddingRows > 0 && <Box height={paddingRows} />}
    </Box>
  );
}
