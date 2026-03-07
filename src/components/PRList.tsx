import React, { useMemo } from "react";
import { Box, Text, useStdout } from "ink";
import type { PullRequest, SectionType, PRKind } from "../types.js";
import { PRRow } from "./PRRow.js";

interface PRListProps {
  prs: PullRequest[];
  selectedIndex: number;
  activeSection: SectionType;
  emptyMessage: string;
}

function getKind(pr: PullRequest, section: SectionType): PRKind {
  if (section === "authored") {
    return pr.isDraft ? "authored draft" : "authored";
  }
  return "mine";
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

  const half = Math.floor(maxVisible / 2);
  let start = Math.max(0, selectedRowIdx - half);
  let end = start + maxVisible;

  if (end > rows.length) {
    end = rows.length;
    start = Math.max(0, end - maxVisible);
  }

  return { start, end };
}

export function PRList({ prs, selectedIndex, activeSection, emptyMessage }: PRListProps) {
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
    <Box flexDirection="column" flexGrow={1} height={maxVisible}>
      {start > 0 && (
        <Box paddingLeft={2}>
          <Text color="gray">...</Text>
        </Box>
      )}
      {visibleRows.map((row, i) => {
        if (row.type === "repo-header") {
          return (
            <Box key={`header-${row.repo}-${start + i}`} paddingLeft={2}>
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
          <Box key={`pr-${row.pr!.repository.nameWithOwner}#${row.pr!.number}`} paddingLeft={2}>
            <PRRow
              pr={row.pr!}
              isSelected={row.flatIndex === selectedIndex}
              kind={getKind(row.pr!, activeSection)}
            />
          </Box>
        );
      })}
      {end < displayRows.length && (
        <Box paddingLeft={2}>
          <Text color="gray">...</Text>
        </Box>
      )}
      {paddingRows > 0 && <Box height={paddingRows} />}
    </Box>
  );
}
