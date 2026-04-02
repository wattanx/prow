import React from "react";
import { test, expect, describe } from "vitest";
import { render } from "ink-testing-library";
import { StatusBar } from "./StatusBar.js";

/** Strip box-drawing characters and collapse whitespace for assertion */
function stripFrame(frame: string): string {
  return frame
    .replace(/[┌┐└┘│─┬┴├┤╭╮╰╯═║╔╗╚╝╠╣╦╩╬]/g, "")
    .replace(/\s+/g, " ")
    .trim();
}

describe("StatusBar", () => {
  test("shows list mode keybindings", () => {
    const { lastFrame } = render(
      <StatusBar mode="list" sortOrder="newest" lastUpdated={null} loading={false} />,
    );
    const frame = stripFrame(lastFrame()!);

    expect(frame).toContain("j/k move");
    expect(frame).toContain("q quit");
  });

  test("shows filter mode keybindings", () => {
    const { lastFrame } = render(
      <StatusBar mode="filter" sortOrder="newest" lastUpdated={null} loading={false} />,
    );
    const frame = stripFrame(lastFrame()!);

    expect(frame).toContain("Space toggle");
    expect(frame).toContain("Esc cancel");
  });

  test("shows loading state", () => {
    const { lastFrame } = render(
      <StatusBar mode="list" sortOrder="newest" lastUpdated={null} loading={true} />,
    );
    const frame = stripFrame(lastFrame()!);
    expect(frame).toContain("Loading");
    expect(frame).not.toContain("Updated");
  });

  test("shows sort order", () => {
    const { lastFrame: newest } = render(
      <StatusBar mode="list" sortOrder="newest" lastUpdated={null} loading={false} />,
    );
    const newestFrame = stripFrame(newest()!);
    expect(newestFrame).toContain("Sort: newest");

    const { lastFrame: oldest } = render(
      <StatusBar mode="list" sortOrder="oldest" lastUpdated={null} loading={false} />,
    );
    const oldestFrame = stripFrame(oldest()!);
    expect(oldestFrame).toContain("Sort: oldest");
  });

  test("hides sort order in filter mode", () => {
    const { lastFrame } = render(
      <StatusBar mode="filter" sortOrder="newest" lastUpdated={null} loading={false} />,
    );
    expect(stripFrame(lastFrame()!)).not.toContain("Sort:");
  });
});
