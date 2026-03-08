import React from "react";
import { render } from "ink";
import { App } from "./app.js";
import { loadConfig } from "./hooks/useConfig.js";
import { VERSION } from "./version.js";
import { selfUpdate } from "./lib/updater.js";

const args = process.argv.slice(2);

if (args.includes("--help") || args.includes("-h")) {
  console.log(`
prow - GitHub PR management TUI

Usage:
  prow            Open the TUI
  prow upgrade    Update to latest version
  prow --help     Show this help
  prow --version  Show version

Keybindings:
  j/↓     Move down
  k/↑     Move up
  Enter   Open PR in browser
  Tab     Switch section (new / stale / mine / authored)
  s       Toggle sort (newest / oldest)
  f       Filter by repository
  r       Refresh
  q       Quit
`);
  process.exit(0);
}

if (args.includes("--version") || args.includes("-v")) {
  console.log(`prow v${VERSION}`);
  process.exit(0);
}

if (args[0] === "upgrade") {
  await selfUpdate();
  process.exit(0);
}

const config = loadConfig();

render(<App config={config} />);
