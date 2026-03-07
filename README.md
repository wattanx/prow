# prow

A TUI application for managing GitHub Pull Requests.

Quickly triage review requests and authored PRs across multiple repositories without leaving the terminal.

## Features

- 4 sections for focused PR triage
- PRs grouped by repository for easy scanning
- Sort by newest or oldest
- Repository filtering
- Auto-polling for live updates
- Vim keybindings + arrow key support

```
 Mine 4 | Authored 3 | New 1 | Stale 2                Updated 13:24
─────────────────────────────────────────────────────────────────────
           │
  > new (1)│  org/repo-a
    stale(2)│    Fix cache invalidation              2h   mine
    mine (4)│    Refactor auth flow                   1d   mine
    authored│
         (3)│  org/repo-b
           │  > Improve loading state               5h   mine
           │    Add error boundary                  3d   mine
           │
─────────────────────────────────────────────────────────────────────
 j/↓ k/↑ move  tab section  ⏎ open  r refresh  s sort  f filter  q quit
```

## Sections

| Section      | Description                                                              |
| ------------ | ------------------------------------------------------------------------ |
| **new**      | Review requests with no reviews yet and updated within the last 48 hours |
| **stale**    | Review requests that have not been updated for more than 48 hours        |
| **mine**     | All PRs where your review is requested (includes both new and stale)     |
| **authored** | All open PRs you created                                                 |

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

| Key       | Action                                         |
| --------- | ---------------------------------------------- |
| `j` / `↓` | Move cursor down                               |
| `k` / `↑` | Move cursor up                                 |
| `Enter`   | Open PR in browser                             |
| `Tab`     | Switch section (new / stale / mine / authored) |
| `s`       | Toggle sort (newest / oldest)                  |
| `f`       | Open repository filter                         |
| `r`       | Refresh data                                   |
| `q`       | Quit                                           |

### Filter mode

| Key       | Action                      |
| --------- | --------------------------- |
| `j` / `↓` | Move cursor down            |
| `k` / `↑` | Move cursor up              |
| `Space`   | Toggle repository selection |
| `Enter`   | Apply filter                |
| `Esc`     | Cancel                      |

## Configuration

Configuration is stored at `~/.config/prow/config.json` (auto-created on first launch).

```json
{
  "pollInterval": 60,
  "defaultSection": "mine"
}
```

| Key              | Description                                             | Default  |
| ---------------- | ------------------------------------------------------- | -------- |
| `pollInterval`   | Auto-refresh interval in seconds                        | `60`     |
| `defaultSection` | Initial section (`new` / `stale` / `mine` / `authored`) | `"mine"` |

## Tech Stack

- [Ink](https://github.com/vadimdemedes/ink) - React for CLI
- [GitHub CLI](https://cli.github.com/) - GitHub API via `gh api graphql`
- [Conf](https://github.com/sindresorhus/conf) - Configuration management
- [tsdown](https://github.com/rolldown/tsdown) - Build tool

## License

MIT
