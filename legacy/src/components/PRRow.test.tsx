import React from "react";
import { test, expect, describe, vi, beforeEach, afterEach } from "vitest";
import { render } from "ink-testing-library";
import { PRRow } from "./PRRow.js";
import { createMockPR } from "../__tests__/helpers.js";

beforeEach(() => {
  vi.useFakeTimers();
  vi.setSystemTime(new Date("2026-04-02T12:00:00Z"));
});

afterEach(() => {
  vi.useRealTimers();
});

describe("PRRow", () => {
  test("renders PR title and author", () => {
    const pr = createMockPR({
      title: "Add new feature",
      author: { login: "alice" },
      updatedAt: "2026-04-02T11:30:00Z",
    });

    const { lastFrame } = render(<PRRow pr={pr} isSelected={false} />);
    const frame = lastFrame()!;

    expect(frame).toContain("Add new feature");
    expect(frame).toContain("alice");
  });

  test("shows selection indicator when selected", () => {
    const pr = createMockPR({ updatedAt: "2026-04-02T11:30:00Z" });

    const { lastFrame: selectedFrame } = render(<PRRow pr={pr} isSelected={true} />);
    expect(selectedFrame()!).toContain(">");

    const { lastFrame: unselectedFrame } = render(<PRRow pr={pr} isSelected={false} />);
    expect(unselectedFrame()!).not.toMatch(/>\s+[✓✗◌-]/);
  });

  describe("relative time formatting", () => {
    test("shows minutes", () => {
      const pr = createMockPR({ updatedAt: "2026-04-02T11:45:00Z" });
      const { lastFrame } = render(<PRRow pr={pr} isSelected={false} />);
      expect(lastFrame()!).toContain("15m");
    });

    test("shows hours", () => {
      const pr = createMockPR({ updatedAt: "2026-04-02T09:00:00Z" });
      const { lastFrame } = render(<PRRow pr={pr} isSelected={false} />);
      expect(lastFrame()!).toContain("3h");
    });

    test("shows days", () => {
      const pr = createMockPR({ updatedAt: "2026-03-30T12:00:00Z" });
      const { lastFrame } = render(<PRRow pr={pr} isSelected={false} />);
      expect(lastFrame()!).toContain("3d");
    });

    test("shows months", () => {
      const pr = createMockPR({ updatedAt: "2026-01-01T00:00:00Z" });
      const { lastFrame } = render(<PRRow pr={pr} isSelected={false} />);
      expect(lastFrame()!).toContain("mo");
    });
  });

  describe("CI status icon", () => {
    test("shows success icon", () => {
      const pr = createMockPR({
        updatedAt: "2026-04-02T11:30:00Z",
        commits: { nodes: [{ commit: { statusCheckRollup: { state: "SUCCESS" } } }] },
      });
      const { lastFrame } = render(<PRRow pr={pr} isSelected={false} />);
      expect(lastFrame()!).toContain("✓");
    });

    test("shows failure icon", () => {
      const pr = createMockPR({
        updatedAt: "2026-04-02T11:30:00Z",
        commits: { nodes: [{ commit: { statusCheckRollup: { state: "FAILURE" } } }] },
      });
      const { lastFrame } = render(<PRRow pr={pr} isSelected={false} />);
      expect(lastFrame()!).toContain("✗");
    });

    test("shows pending icon", () => {
      const pr = createMockPR({
        updatedAt: "2026-04-02T11:30:00Z",
        commits: { nodes: [{ commit: { statusCheckRollup: { state: "PENDING" } } }] },
      });
      const { lastFrame } = render(<PRRow pr={pr} isSelected={false} />);
      expect(lastFrame()!).toContain("◌");
    });

    test("shows dash when no status", () => {
      const pr = createMockPR({
        updatedAt: "2026-04-02T11:30:00Z",
        commits: { nodes: [{ commit: { statusCheckRollup: null } }] },
      });
      const { lastFrame } = render(<PRRow pr={pr} isSelected={false} />);
      expect(lastFrame()!).toContain("-");
    });
  });
});
