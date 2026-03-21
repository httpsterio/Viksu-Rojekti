# Rojekti - Development Guide

Rojekti is a local-first kanban board built with Tauri 2. Cards are stored as Markdown files with YAML frontmatter. The app provides a GUI and a CLI. See `docs/PROJECT_PLAN.md` for the full specification.

---

## 1. Command Delegation (CRITICAL)

The dev environment is WSL2 (Ubuntu). The agent edits files from WSL2. All build and runtime commands are executed by the user on the Windows side manually.

Rules:

- Never run `cmd.exe`, `npm`, `cargo`, or `npx` from the agent shell. These do not work from WSL2 for this project.
- When a build or install step is needed, provide the exact command and state what output or confirmation is needed before proceeding.
- Do not manage `node_modules`. If deps are missing, ask the user to run the install command.

## 2. Environment

- Project path: `/mnt/d/MISC/PROJECTS/Rojekti/`
- The project lives on the Windows filesystem. Do not move files to the WSL2 home directory.
- Editing: use WSL2 unix tools (rg, fd, jq, tree).
- Building: user runs `npm run dev` or `npm run build` on Windows side.
- Rust, Node, npm, and Tauri CLI are installed on Windows.

## 3. Data folder structure

All Rojekti data lives in a `rojekti/` subfolder relative to the executable. Not in the project root.

```
my-project/
├── Rojekti.exe
└── rojekti/
    ├── rojekti.config.yaml       # Board config (lanes, epics, tags, priorities)
    ├── rojekti.index.yaml       # Auto-generated card summary (never edit manually)
    └── cards/           # One .md file per card
        ├── ROJ-001.md
        ├── ROJ-002.md
        └── ...
```

The exe looks for `rojekti/board.yaml` relative to the working directory, then relative to the exe location. If not found, the GUI shows an init dialog.

## 4. Card file format

Cards use YAML frontmatter + Markdown body:

```markdown
---
id: ROJ-001
title: Add receipt OCR pipeline
status: backlog
epic: receipt-handling
tags:
  - feature
priority: 4
position: 1.0
created: 2026-03-10
---

Description content here. Full markdown supported.
```

The frontmatter is the structured data. The body below `---` is the description. When writing card files, always preserve the body when updating frontmatter fields.

## 5. CLI architecture

The CLI is handled in `main.rs` BEFORE Tauri initializes. This is critical.

```rust
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        // CLI mode: handle command, flush stdout, exit
        // Tauri is never initialized
        cli::handle(&args);
        std::io::stdout().flush().unwrap();
        std::process::exit(0);
    }

    // GUI mode: no CLI args, launch Tauri
    rojekti_lib::run();
}
```

Do NOT put CLI dispatch inside Tauri's `.setup()` hook. That causes:

- Shell prompt returning before output finishes (Windows treats it as GUI app)
- Console window flash/ghost errors from WebView2 cleanup
- 500ms+ startup delay from unnecessary Tauri initialization

The binary uses console subsystem. In GUI mode, the console is hidden programmatically via `FreeConsole()` after checking `GetConsoleProcessList`. This causes a brief console flash on double-click which is acceptable.

## 6. Coding conventions

### Rust

- All structs in `models.rs`. Add `#[serde(rename_all = "camelCase")]` to every struct sent to the frontend.
- All file I/O in `storage.rs`. Commands in `commands.rs` call storage functions. They never touch the filesystem directly.
- Index logic in `index.rs`. CLI handlers in `cli.rs`.
- Error handling: always return `Result<T, String>`. No `unwrap()` or `expect()` in command handlers. Convert errors with `.map_err(|e| format!("context: {}", e))`.
- Use `PathBuf` and `.join()` for paths. Never string concatenation.
- Frontmatter parsing: split on `---` with `splitn(3, "---")`. First element is empty, second is YAML, third is markdown body.

### TypeScript / Vue

- `<script setup lang="ts">` in all SFCs. Composition API only.
- All state in `composables/useBoard.ts`. Components never call `invoke()` directly.
- No Pinia, no Vuex, no Vue Router.
- Types in `src/types/index.ts` matching the Rust structs (camelCase field names).
- Use `ref()` and `computed()`. No `reactive()` for top-level state.

### UI

- PrimeVue (Aura theme) for form components: Button, Select, Dialog, InputText, Tag, Toast, Textarea.
- PrimeIcons via class names: `<i class="pi pi-plus"></i>`.
- md-editor-v3 for the description field. Two modes:
  - View (default): `previewOnly` mode, rendered markdown.
  - Edit: full editor with toolbar.
- SortableJS for drag-and-drop in lanes.
- Custom CSS for board layout. No Tailwind.
- CSS variables for theming. Dark/light via `prefers-color-scheme`.

### What not to do

- Do not add dependencies without asking first.
- Do not use the Tauri filesystem plugin. All file I/O goes through custom commands backed by `std::fs`.
- Do not add `tauri-plugin-opener`. It has caused build failures.
- Do not put CLI handling in the Tauri setup hook.
- Do not use `any` in TypeScript.
- Do not write comments explaining what code does. Only comment the "why" when non-obvious.

## 7. Known issues and pitfalls

- `@/` path alias is configured in both `vite.config.ts` and `tsconfig.json`. If imports break, check both.
- SortableJS manipulates the DOM directly which conflicts with Vue's virtual DOM. After a drag, update Vue state and let Vue re-render. Do not trust SortableJS DOM state.
- Position values are floats. After many reorderings, values can get very close together. Trigger renormalization (reassign 1.0, 2.0, 3.0...) when `Math.abs(a - b) < 0.001`.
- `serde_yaml` serializes `None` as `null`. Use `#[serde(skip_serializing_if = "Option::is_none")]` on optional fields.
- `serde_yaml` serializes empty `Vec` as `[]`. Use `#[serde(default, skip_serializing_if = "Vec::is_empty")]` to omit empty lists.
- When writing card files, ensure a blank line between closing `---` and the body: `format!("---\n{}---\n\n{}\n", yaml, body)`.

## 8. Reference

- Full project specification: `docs/PROJECT_PLAN.md`
- Board config schema: see section 3.1 in PROJECT_PLAN.md
- Card file schema: see section 3.2 in PROJECT_PLAN.md
- Tauri command signatures: see section 4.4 in PROJECT_PLAN.md
- Component tree and UI details: see section 7 in PROJECT_PLAN.md
