# Rojekti

A local-first kanban board built with Tauri 2. Cards are stored as plain Markdown files with YAML frontmatter — no database, so the board is just a folder you can read, edit by hand, and version with git.

## Requirements

- Windows 10 or later (only supported platform right now)
- [Node.js](https://nodejs.org) + npm
- [Rust](https://rustup.rs) (stable toolchain)
- Tauri's Windows prerequisites (MSVC Build Tools, WebView2 — usually already present on Windows 10/11)

## Build from source

```sh
git clone git@github.com:httpsterio/Viksu-Rojekti.git
cd Viksu-Rojekti
npm install
npm run tauri dev
```

Edited from WSL2, but build and dev commands must be run from a Windows shell (PowerShell/cmd), not WSL — native bindings won't build there. `npm run build` produces `Rojekti.exe`.

## First launch

Run the app from the folder you want to use as a project root:

```sh
./Rojekti.exe
```

If no `rojekti/` folder exists there, Rojekti shows a welcome screen asking for a **Board Name** and a **Card Prefix** (used in card IDs, e.g. `ROJ-001`). Confirming creates the folder below with five default lanes (Backlog, Todo, In Progress, Review, Done) and a default priority scale (Critical → Low).

## Data folder

```
my-project/
├── Rojekti.exe
└── rojekti/
    ├── rojekti.config.yaml    # statuses, epics, tags, priorities
    ├── rojekti.index.yaml     # auto-generated card index — don't edit by hand
    ├── rojekti.state.yaml     # per-user UI state (theme, collapsed lanes, filters)
    └── cards/
        ├── roj-001.md
        └── ...
```

A card is Markdown with YAML frontmatter:

```markdown
---
id: roj-001
title: Add receipt OCR pipeline
status: Todo
epic: Board
tags:
  - Feature
priority: 3
position: 1.0
created: 2026-03-10
---

Full Markdown body here.
```

## Usage

- Drag cards between lanes — status and order are written straight back to the card's frontmatter.
- Switch between board view and epic-grouped view from the top bar.
- Board Settings (top bar) manages statuses, epics, tags, and priorities. Renaming any of them safely rewrites every affected card.
- Rojekti watches the `cards/` folder, so edits made outside the app (hand-editing a `.md` file, a git pull) are picked up live.

## CLI

A CLI (`init`, `list`, `show`, `create`, `rebuild-index`, `reorder`) is available for scripting against the same files. Run `Rojekti.exe --help`.

## License

Source-available, view and run only — no modification, redistribution, or commercial use. See [LICENSE.md](LICENSE.md).
