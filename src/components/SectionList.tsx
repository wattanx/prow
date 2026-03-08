import React from "react";
import { Box, Text } from "ink";
import type { SectionType } from "../types.js";

interface SectionListProps {
  activeSection: SectionType;
  counts: {
    new: number;
    stale: number;
    mine: number;
    authored: number;
  };
}

const SECTIONS: SectionType[] = ["new", "stale", "mine", "authored"];

export function SectionList({ activeSection, counts }: SectionListProps) {
  return (
    <Box gap={1} paddingX={1} borderBottom borderStyle="single">
      {SECTIONS.map((section) => {
        const isActive = section === activeSection;
        return (
          <Text key={section} bold={isActive} color={isActive ? "blue" : "gray"} inverse={isActive}>
            {` ${section} (${counts[section]}) `}
          </Text>
        );
      })}
    </Box>
  );
}

export { SECTIONS };
