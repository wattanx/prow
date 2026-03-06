import React from "react";
import { render } from "ink";
import { App } from "./app.js";
import { loadConfig } from "./hooks/useConfig.js";

const args = process.argv.slice(2);

if (args.includes("--help") || args.includes("-h")) {
  console.log(`
prow - GitHub PR management TUI

Usage:
  prow            Open the TUI
  prow --help     Show this help
  prow --version  Show version

Keybindings:
  j/↓     Move down
  k/↑     Move up
  Enter   Open PR in browser
  Tab     Switch tab (Created / Review Requested)
  f       Filter by repository
  r       Refresh
  q       Quit
`);
  process.exit(0);
}

if (args.includes("--version") || args.includes("-v")) {
  console.log("prow v0.1.0");
  process.exit(0);
}

const config = loadConfig();

render(<App config={config} />);
