import React from "react";
import { test, expect, describe } from "vitest";
import { render } from "ink-testing-library";
import { RepoFilter } from "./RepoFilter.js";

describe("RepoFilter", () => {
  const repos = ["owner/repo-a", "owner/repo-b", "owner/repo-c"];

  test("renders all repos with 'All' option", () => {
    const { lastFrame } = render(
      <RepoFilter allRepos={repos} selectedRepos={new Set()} cursorIndex={0} />,
    );
    const frame = lastFrame()!;

    expect(frame).toContain("Filter by repository:");
    expect(frame).toContain("All");
    expect(frame).toContain("owner/repo-a");
    expect(frame).toContain("owner/repo-b");
    expect(frame).toContain("owner/repo-c");
  });

  test("shows 'All' checked when selectedRepos is empty", () => {
    const { lastFrame } = render(
      <RepoFilter allRepos={repos} selectedRepos={new Set()} cursorIndex={0} />,
    );
    const frame = lastFrame()!;

    // All is checked
    expect(frame).toMatch(/\[x\] All/);
    // Individual repos are also checked when All is selected
    expect(frame).toMatch(/\[x\] owner\/repo-a/);
  });

  test("shows cursor at specified index", () => {
    const { lastFrame } = render(
      <RepoFilter allRepos={repos} selectedRepos={new Set()} cursorIndex={2} />,
    );
    const frame = lastFrame()!;

    // Cursor should be on repo-b (index 2 = second repo)
    expect(frame).toContain("> [x] owner/repo-b");
  });

  test("shows unchecked repos when filtering", () => {
    const selected = new Set(["owner/repo-a"]);
    const { lastFrame } = render(
      <RepoFilter allRepos={repos} selectedRepos={selected} cursorIndex={0} />,
    );
    const frame = lastFrame()!;

    // All is unchecked
    expect(frame).toMatch(/\[ \] All/);
    // repo-a is checked
    expect(frame).toMatch(/\[x\] owner\/repo-a/);
    // repo-b is unchecked
    expect(frame).toMatch(/\[ \] owner\/repo-b/);
  });
});
