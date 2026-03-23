# Rojekti — Development Guide

Rojekti is a local-first kanban board built with Tauri 2. Cards are stored as Markdown files with YAML frontmatter. The app has a GUI and a CLI.

---

## Behaviour

**Wait to be asked.** A session starting with context, a summary, or prior work is not an instruction to act. Wait for an explicit request.

**Ask before implementing.** For anything beyond a trivial, isolated fix, state what you plan to do and wait for confirmation before touching files. This applies especially to multi-file changes.

**Ask when something is unclear.** If the intent behind a request is ambiguous, or if there are meaningful design choices to make, stop and ask. Do not assume and proceed.

**Check git status before editing.** Run `git status` before making any changes. If there are uncommitted changes, understand what they are. Do not write on top of in-progress work without flagging it first.

**Trace the full user journey when verifying.** Do not read functions in isolation. Follow a user action from the UI through the frontend state, into the Tauri command, through storage, and to the final state on disk. Assumptions made early in a call chain often break at handoff boundaries — this is only visible by tracing the full flow.

**Never commit without being asked.** Do not stage or commit files unless explicitly instructed.

---

## Dev environment

- WSL2 is used for editing and git
- All build and run commands (`npm run dev`, `npm run build`, `cargo build`) must be run by the user on the Windows side — do not attempt to run them from the agent shell
- Do not manage `node_modules`. If deps are missing, tell the user what to run.
- Project path: `/mnt/d/MISC/PROJECTS/Rojekti/`

---

## Data folder structure

All Rojekti data lives in a `rojekti/` subfolder relative to the project directory.

```
my-project/
├── Rojekti.exe
└── rojekti/
    ├── rojekti.config.yaml    # Board config (statuses, epics, tags, priorities)
    ├── rojekti.index.yaml     # Auto-generated card index (never edit manually)
    ├── rojekti.state.yaml     # Ephemeral user preferences (theme, collapsed lanes, etc.)
    └── cards/
        ├── roj-001.md
        ├── roj-002.md
        └── ...
```

---

## Card file format

```markdown
---
id: roj-001
title: Add receipt OCR pipeline
status: TO DO
epic: Board
tags:
  - Feature
priority: 3
position: 1.0
created: 2026-03-10
---

Description content here. Full markdown supported.
```

- `status` stores the status **name** directly (not an ID)
- `epic` and `tags` store names directly
- `priority` is a number 1–5 (0 = unset)
- `position` is a float used for ordering within a status lane
- Always preserve the body when updating frontmatter fields
- Blank line required between closing `---` and body

---

## Code structure

### Rust (`src-tauri/src/`)

| File | Purpose |
|------|---------|
| `models.rs` | All structs. Every struct sent to the frontend has `#[serde(rename_all = "camelCase")]` |
| `storage.rs` | All file I/O. Commands never touch the filesystem directly. |
| `commands.rs` | Tauri command handlers. Call storage functions, never `std::fs` directly. |
| `index.rs` | Index rebuild logic |
| `cli.rs` | CLI command handlers |
| `watcher.rs` | File watcher |
| `lib.rs` | App startup, watcher init, WAL recovery on startup |

Rules:
- Return `Result<T, String>` from all command handlers. No `unwrap()` or `expect()`.
- Use `.map_err(|e| format!("context: {}", e))` for error conversion.
- Use `PathBuf` and `.join()` for all paths. No string concatenation.
- `#[serde(skip_serializing_if = "Option::is_none")]` on optional fields.
- `#[serde(default, skip_serializing_if = "Vec::is_empty")]` on optional vec fields.

### TypeScript / Vue (`src/`)

- `<script setup lang="ts">` in all SFCs. Composition API only.
- All board state in `src/composables/useBoard.ts`. Components never call `invoke()` directly.
- Types in `src/types/index.ts` matching Rust structs (camelCase).
- No `any` in TypeScript.
- No Pinia, no Vuex, no Vue Router.

### UI

- PrimeVue (Aura theme) for components: Button, Select, Dialog, InputText, Tag, Toast, etc.
- PrimeIcons via class names: `<i class="pi pi-plus"></i>`
- `md-editor-v3` for card description (view mode: `previewOnly`, edit mode: full editor)
- `@formkit/drag-and-drop` for drag-and-drop in lanes and settings
- Custom CSS for board layout. No Tailwind. CSS variables for theming.

### Comments

No comments explaining what code does. Only comment the *why*, and only when it's genuinely non-obvious.

---

## What not to do

- Do not add dependencies without asking first
- Do not use the Tauri filesystem plugin — all I/O goes through custom commands backed by `std::fs`
- Do not add `tauri-plugin-opener` — has caused build failures
- Do not put CLI handling in the Tauri setup hook
- Do not use `any` in TypeScript
- Do not run build commands from WSL

---

## Known pitfalls

- **`@/` path alias** is configured in both `vite.config.ts` and `tsconfig.json`. If imports break, check both.
- **Drag-and-drop and Vue reactivity**: `@formkit/drag-and-drop` manipulates state directly. Use its `values` ref as the source of truth during drag, sync back to `localConfig` via `onSort`.
- **Position float precision**: after many reorderings, position values can get very close together. Trigger renormalization (1.0, 2.0, 3.0...) when `Math.abs(a - b) < 0.001`.
- **`serde_yaml` and None**: serializes `None` as `null` unless `skip_serializing_if = "Option::is_none"` is set.
- **Status names are the identifier**: statuses no longer have an `id` field. Cards store the status name directly. Renaming a status requires updating all card files via the WAL pattern in `storage.rs`.
- **Rename safety**: epic, tag, and status renames use a write-ahead log (`pending_rename` field on the struct). On startup, `lib.rs` checks for and completes any interrupted renames. Do not bypass this pattern.
