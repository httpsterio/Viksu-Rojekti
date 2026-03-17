# Rojekti - Development Guide

Rojekti is a local-first kanban board built with Tauri 2. It stores tickets as Markdown files with YAML frontmatter in your project directory.

## 1. Command Delegation Protocol (CRITICAL)

The development environment is a hybrid WSL2 (Linux) and Windows setup. The following rules are absolute:

- **No `cmd.exe` Execution:** The AI agent must **NEVER** attempt to run commands via `cmd.exe` or `npm` through its internal shell tools. All Windows-side execution is strictly delegated to the user.
- **How to Delegate:** When a build, dev, or package management task is required, the agent must:
    1.  Provide the **exact command string** for the user to run on the Windows side.
    2.  State clearly **what output or confirmation** is needed from the user to proceed to the next step.
- **Node Modules:** The agent must not attempt to manage `node_modules`. If dependencies are missing or the environment is broken, the agent must ask the user to resolve it on the Windows side.

## 2. Environment & Architecture

- **Editing:** Use WSL2 (Ubuntu) for file manipulation using standard unix tools (`rg`, `fd`, `jq`, etc.).
- **Filesystem:** The project **MUST** live on the Windows filesystem (e.g., `/mnt/d/MISC/PROJECTS/Rojekti/`). Do not move it to the WSL2 home directory as it breaks path resolution for Tauri.
- **Console Subsystem (CLI Sync):** This app is built as a **Console Subsystem** application to ensure terminal synchronization for CLI commands.
    - In **GUI mode** (no arguments), the console window is programmatically hidden in `main.rs`.
    - In **CLI mode** (with arguments), it operates as a standard console app.

## 3. Project Discovery
The backend (`lib.rs`) determines the project root by searching for `board.yaml` in this order:
1. Current working directory (CWD).
2. Parent of CWD.
3. Directory containing the executable.
4. Parent of the executable directory.

## 4. Coding Conventions

### Backend (Rust)
- **Public types:** `models.rs` (use `#[serde(rename_all = "camelCase")]`).
- **File I/O:** `storage.rs`.
- **Logic:** `index.rs` (indexing), `cli.rs` (CLI handlers), `commands.rs` (IPC).
- **Error Handling:** Return `Result<T, String>`. Avoid `unwrap()` or `expect()` in command handlers.

### Frontend (Vue 3 + TypeScript)
- **State:** `useBoard.ts` is the single source of truth. Components call its functions, never `invoke` directly.
- **UI:** PrimeVue (Aura theme) + custom CSS in `src/styles/main.css`.
- **Editor:** `md-editor-v3` for Markdown editing/preview.
- **Drag & Drop:** SortableJS in `Lane.vue`.

## 5. Known Pitfalls
- **Tauri Plugin Opener:** Do not add this plugin; it has caused build/initialization failures.
- **Path Aliases:** The `@/` alias is configured in both `vite.config.ts` and `tsconfig.json`.
- **CLI Exits:** CLI subcommands must call `std::process::exit(0)` in the `setup` hook to prevent the GUI event loop from starting.

## 6. Project Data Structure
```
my-project/
├── board.yaml       # Configuration (Lanes, Epics, Tags)
├── index.yaml       # Auto-generated ticket summary
└── tickets/         # Individual .md ticket files
```
