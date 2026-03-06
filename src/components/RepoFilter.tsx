import React from "react";
import { Box, Text } from "ink";

interface RepoFilterProps {
  allRepos: string[];
  selectedRepos: Set<string>;
  cursorIndex: number;
}

export function RepoFilter({ allRepos, selectedRepos, cursorIndex }: RepoFilterProps) {
  const isAllSelected = selectedRepos.size === 0;

  return (
    <Box flexDirection="column" paddingLeft={2} paddingTop={1}>
      <Text bold>Filter by repository:</Text>
      <Box paddingTop={1} flexDirection="column">
        <Box>
          <Text color={cursorIndex === 0 ? "blue" : undefined}>
            {cursorIndex === 0 ? ">" : " "} [{isAllSelected ? "x" : " "}] All
          </Text>
        </Box>
        {allRepos.map((repo, index) => {
          const itemIndex = index + 1;
          const isChecked = isAllSelected || selectedRepos.has(repo);
          return (
            <Box key={repo}>
              <Text color={cursorIndex === itemIndex ? "blue" : undefined}>
                {cursorIndex === itemIndex ? ">" : " "} [{isChecked ? "x" : " "}] {repo}
              </Text>
            </Box>
          );
        })}
      </Box>
    </Box>
  );
}
