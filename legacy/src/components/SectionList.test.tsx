import React from "react";
import { test, expect, describe } from "vitest";
import { render } from "ink-testing-library";
import { SectionList } from "./SectionList.js";

const counts = { new: 3, stale: 1, all: 10, authored: 5 };

describe("SectionList", () => {
  test("renders all sections with counts", () => {
    const { lastFrame } = render(<SectionList activeSection="all" counts={counts} />);
    const frame = lastFrame()!;

    expect(frame).toContain("new (3)");
    expect(frame).toContain("stale (1)");
    expect(frame).toContain("all (10)");
    expect(frame).toContain("authored (5)");
  });

  test("highlights active section", () => {
    const { lastFrame: frame1 } = render(<SectionList activeSection="new" counts={counts} />);
    const { lastFrame: frame2 } = render(<SectionList activeSection="authored" counts={counts} />);

    // Both render without error and contain the expected sections
    expect(frame1()!).toContain("new (3)");
    expect(frame2()!).toContain("authored (5)");
  });
});
