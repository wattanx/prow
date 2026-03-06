# prow

A TUI application for managing GitHub Pull Requests.

Browse PRs across multiple repositories, switch between created and review-requested views, and open them in your browser.

## Features

- Tab switching between your created PRs and review-requested PRs
- Cross-repository PR listing
- CI status, review state, labels, and update time at a glance
- Repository filtering
- Auto-polling for live updates
- Vim keybindings + arrow key support

```
┌───────────────────────────────────────────────────┐
│  [Created (3)]  [Review Requested (2)]            │
│───────────────────────────────────────────────────│
│  Repo          Title              CI  Rev  Age    │
│───────────────────────────────────────────────────│
│> owner/repo-a  fix: login bug     ✓   2/3  2h    │
│  owner/repo-b  feat: new api      ✗   0/1  1d    │
│  owner/repo-a  chore: update deps ✓   1/1  3d    │
│───────────────────────────────────────────────────│
│  ↑↓/jk move  ⏎ open  Tab switch  f filter  q quit│
└───────────────────────────────────────────────────┘
```

## Prerequisites

- Node.js >= 20
- [GitHub CLI](https://cli.github.com/) (`gh`) installed and authenticated

```bash
gh auth login
```

## Install

```bash
pnpm install
pnpm build
```

## Usage

```bash
# Start the TUI
node dist/index.mjs

# Or
./bin/prow.js
```

```bash
prow --help     # Show help
prow --version  # Show version
```

## Keybindings

| Key | Action |
| --- | --- |
| `j` / `↓` | Move cursor down |
| `k` / `↑` | Move cursor up |
| `Enter` | Open PR in browser |
| `Tab` | Switch tab (Created / Review Requested) |
| `f` | Open repository filter |
| `r` | Refresh data |
| `q` | Quit |

### Filter mode

| Key | Action |
| --- | --- |
| `j` / `↓` | Move cursor down |
| `k` / `↑` | Move cursor up |
| `Space` | Toggle repository selection |
| `Enter` | Apply filter |
| `Esc` | Cancel |

## Configuration

Configuration is stored at `~/.config/prow/config.json` (auto-created on first launch).

```json
{
  "columns": ["repo", "title", "ci", "reviews", "labels", "updatedAt"],
  "pollInterval": 60,
  "defaultTab": "created"
}
```

| Key | Description | Default |
| --- | --- | --- |
| `columns` | Columns to display | `["repo", "title", "ci", "reviews", "labels", "updatedAt"]` |
| `pollInterval` | Auto-refresh interval in seconds | `60` |
| `defaultTab` | Initial tab (`created` / `reviewRequested`) | `"created"` |

## Tech Stack

- [Ink](https://github.com/vadimdemedes/ink) - React for CLI
- [Octokit GraphQL](https://github.com/octokit/graphql.js) - GitHub GraphQL API
- [Conf](https://github.com/sindresorhus/conf) - Configuration management
- [tsdown](https://github.com/rolldown/tsdown) - Build tool

## License

MIT
