import React from "react";
import { Box, Text } from "ink";
import type { TabType } from "../types.js";

interface TabBarProps {
  activeTab: TabType;
  createdCount: number;
  reviewRequestedCount: number;
}

export function TabBar({
  activeTab,
  createdCount,
  reviewRequestedCount,
}: TabBarProps) {
  return (
    <Box gap={2} paddingX={1} paddingY={1}>
      <Text
        bold={activeTab === "created"}
        color={activeTab === "created" ? "blue" : "gray"}
        inverse={activeTab === "created"}
      >
        {" "}
        Created ({createdCount}){" "}
      </Text>
      <Text
        bold={activeTab === "reviewRequested"}
        color={activeTab === "reviewRequested" ? "blue" : "gray"}
        inverse={activeTab === "reviewRequested"}
      >
        {" "}
        Review Requested ({reviewRequestedCount}){" "}
      </Text>
    </Box>
  );
}
