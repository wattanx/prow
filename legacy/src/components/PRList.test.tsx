import React from "react";
import { test, expect, describe } from "vitest";
import { render } from "ink-testing-library";
import { PRList } from "./PRList.js";
import { createMockPR } from "../__tests__/helpers.js";

describe("PRList", () => {
  test("shows loading message when loading with no PRs", () => {
    const { lastFrame } = render(
      <PRList prs={[]} selectedIndex={0} emptyMessage="No PRs" loading={true} />,
    );
    expect(lastFrame()!).toContain("Loading...");
  });

  test("shows empty message when no PRs and not loading", () => {
    const { lastFrame } = render(
      <PRList prs={[]} selectedIndex={0} emptyMessage="No review requests" loading={false} />,
    );
    expect(lastFrame()!).toContain("No review requests");
  });

  test("renders PRs grouped by repository", () => {
    const prs = [
      createMockPR({
        title: "PR in repo A",
        repository: { nameWithOwner: "owner/repo-a" },
        updatedAt: "2026-04-02T10:00:00Z",
      }),
      createMockPR({
        title: "PR in repo B",
        repository: { nameWithOwner: "owner/repo-b" },
        updatedAt: "2026-04-02T09:00:00Z",
      }),
    ];

    const { lastFrame } = render(
      <PRList prs={prs} selectedIndex={0} emptyMessage="No PRs" loading={false} />,
    );
    const frame = lastFrame()!;

    expect(frame).toContain("owner/repo-a");
    expect(frame).toContain("PR in repo A");
    expect(frame).toContain("owner/repo-b");
    expect(frame).toContain("PR in repo B");
  });

  test("highlights selected PR", () => {
    const prs = [
      createMockPR({
        title: "First PR",
        updatedAt: "2026-04-02T10:00:00Z",
      }),
      createMockPR({
        title: "Second PR",
        updatedAt: "2026-04-02T09:00:00Z",
      }),
    ];

    const { lastFrame } = render(
      <PRList prs={prs} selectedIndex={1} emptyMessage="No PRs" loading={false} />,
    );
    const frame = lastFrame()!;

    expect(frame).toContain("First PR");
    expect(frame).toContain("Second PR");
  });
});
