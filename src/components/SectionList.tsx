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
    <Box flexDirection="column" width={16} paddingTop={1}>
      {SECTIONS.map((section) => {
        const isActive = section === activeSection;
        return (
          <Box key={section}>
            <Text bold={isActive} color={isActive ? "blue" : undefined}>
              {isActive ? "> " : "  "}
              {section} ({counts[section]})
            </Text>
          </Box>
        );
      })}
    </Box>
  );
}

export { SECTIONS };
