# Development Instructions: Rojekti - Local Kanban Board

Read this document fully before writing any code. Follow every convention listed here. If something is ambiguous, ask before guessing.

## Project summary

Rojekti is a local-first kanban board built with Tauri 2. Tickets are Markdown files with YAML frontmatter stored in a project directory. The app provides a GUI for humans and a CLI for LLM agents. See `docs/PROJECT_PLAN.md` for the full specification.

Primary target: Windows. Secondary: macOS, Linux.

---

## 1. Development environment

### Hybrid WSL2 / Windows setup

The development environment is WSL2 (Ubuntu). The LLM agent operates in WSL2 using unix tools (rg, fd, jq, tree, etc.) for file editing, searching, and navigation. However, Tauri targets the Windows WebView2 runtime, so all build and dev commands are delegated to Windows via `cmd.exe`.

The project lives on the Windows filesystem so both sides can access it without cross-filesystem performance issues:

```
/mnt/d/MISC/PROJECTS/Rojekti/
```

**Important**: the project must stay on the Windows filesystem (`/mnt/d/...`), not in the WSL2 home directory. Windows-side processes cannot efficiently access the WSL2 filesystem and path resolution breaks.

### How it works

- The agent edits files from WSL2 using standard unix tools.
- `npm run dev` and `npm run build` delegate to Windows via `cmd.exe`.
- Rust, Node, npm, and Tauri CLI are installed on the **Windows side**.
- WSL2 needs Node/npm installed only to run the npm scripts that delegate to Windows.

### package.json scripts

```json
{
  "scripts": {
    "vite:dev": "vite",
    "vite:build": "vite build",
    "dev": "cmd.exe /c npx tauri dev",
    "build": "cmd.exe /c npx tauri build",
    "preview": "vite preview"
  }
}
```

The `vite:dev` and `vite:build` scripts are called by Tauri via `beforeDevCommand` and `beforeBuildCommand` in `tauri.conf.json`. The `dev` and `build` scripts are what you run from the terminal.

### Prerequisites

**Windows side** (must be in Windows PATH):

1. **Rust** (via rustup): https://rustup.rs/
   - `rustup default stable` after install.
   - Verify from PowerShell: `rustc --version` (1.75+)

2. **Node.js** (LTS, 20+): https://nodejs.org/
   - Verify from PowerShell: `node --version` and `npm --version`

3. **Visual Studio Build Tools** (or full Visual Studio):
   - Workload: "Desktop development with C++"
   - Required for Rust compilation on Windows.

4. **WebView2 Runtime**: pre-installed on Windows 10 (1803+) and Windows 11.

**WSL2 side**:

1. **Node.js** (LTS, 20+): needed to run npm scripts.
   - Verify: `node --version` and `npm --version`

2. Standard unix tools: `rg`, `fd`, `jq`, `tree` (already installed).

### Verify setup

From WSL2:

```bash
# WSL2 side
node --version
npm --version

# Windows side (via cmd.exe)
cmd.exe /c "rustc --version"
cmd.exe /c "cargo --version"
cmd.exe /c "node --version"
```

All must succeed before proceeding.

### Path note

When running commands from WSL2 that delegate to Windows, the working directory translates automatically (e.g., `/mnt/d/MISC/PROJECTS/Rojekti` becomes `D:\MISC\PROJECTS\Rojekti` on the Windows side). This works transparently for `cmd.exe /c` calls from the project directory.

---

## 2. Project scaffolding

### Initialize the project

From WSL2, in the project directory:

```bash
cd /mnt/d/MISC/PROJECTS/Rojekti
cmd.exe /c "npm create tauri-app@latest . -- --template vue-ts"
```

When prompted:
- Package manager: npm
- UI template: Vue
- TypeScript: Yes

### Install dependencies

```bash
cmd.exe /c "npm install"
```

Then add project-specific dependencies:

```bash
# Frontend UI
cmd.exe /c "npm install primevue @primevue/themes primeicons"

# Drag and drop
cmd.exe /c "npm install sortablejs"
cmd.exe /c "npm install -D @types/sortablejs"

# Markdown editor (Vue 3 native)
cmd.exe /c "npm install md-editor-v3"

# Markdown rendering (for any non-editor rendering needs)
cmd.exe /c "npm install marked"
cmd.exe /c "npm install -D @types/marked"
```

### Rust dependencies

Edit `src-tauri/Cargo.toml` and set these dependencies:

```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-cli = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
chrono = { version = "0.4", features = ["serde"] }
```

Verify Rust compiles:

```bash
cd src-tauri
cmd.exe /c "cargo check"
cd ..
```

### Verify the scaffold works

```bash
npm run dev
```

This runs `cmd.exe /c npx tauri dev` under the hood. A window should open with the default Tauri + Vue template. If this fails, fix the environment before proceeding. Do not write any application code until the scaffold runs.

---

## 3. Project structure

Follow this structure exactly. Do not rename files or reorganize without explicit instruction.

```
kanban/
├── docs/
│   └── PROJECT_PLAN.md          # Full spec (read-only reference)
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── src/
│       ├── main.rs              # Entry point: CLI dispatch or GUI launch
│       ├── cli.rs               # CLI subcommand handlers
│       ├── commands.rs          # Tauri IPC command handlers
│       ├── models.rs            # All struct definitions
│       ├── storage.rs           # File I/O: read/write frontmatter+md, directory scan
│       └── index.rs             # Index rebuild logic
├── src/
│   ├── main.ts                  # Vue app entry
│   ├── App.vue                  # Root component
│   ├── composables/
│   │   └── useBoard.ts          # All application state, Tauri invoke wrappers
│   ├── components/
│   │   ├── TopBar.vue
│   │   ├── BoardView.vue
│   │   ├── Lane.vue
│   │   ├── TicketCard.vue
│   │   ├── EpicView.vue
│   │   ├── EpicGroup.vue
│   │   ├── TicketModal.vue
│   │   ├── BoardSettingsModal.vue
│   │   └── Toast.vue
│   ├── utils/
│   │   ├── position.ts          # Position calculation helpers
│   │   └── filters.ts           # Ticket filtering logic
│   ├── types/
│   │   └── index.ts             # TypeScript interfaces matching Rust structs
│   └── styles/
│       └── main.css             # Global styles, CSS variables, theme
├── index.html
├── vite.config.ts
├── package.json
└── tsconfig.json
```

---

## 4. Coding conventions

### General

- Use TypeScript for all frontend code. No `any` types. Define interfaces for every data structure.
- Use Rust for all backend code. No `unwrap()` in command handlers. Always return `Result<T, String>`.
- Comments only where the "why" is not obvious. No comments restating what the code does.
- No unused imports. No unused variables. No dead code.
- Prefer early returns over nested conditionals.

### Rust

- All public types go in `models.rs`. Do not define structs inline in other files.
- All file I/O goes through `storage.rs`. Commands in `commands.rs` call storage functions. They do not touch the filesystem directly.
- Use `serde_yaml` for YAML. Use `chrono::Local::now().format("%Y-%m-%d")` for dates.
- Every Tauri command function is annotated with `#[tauri::command]` and registered in `main.rs` via `generate_handler![]`.
- Error handling: return `Result<T, String>`. Convert errors with `.map_err(|e| format!("context: {}", e))`. Do not use `.unwrap()` or `.expect()` in command handlers.
- Use `std::path::PathBuf` for all file paths. Construct paths with `.join()`, never with string concatenation or format strings.
- Add `#[serde(rename_all = "camelCase")]` to every struct sent to the frontend. This ensures automatic field name conversion (e.g., `next_id` in Rust becomes `nextId` in TypeScript).

Example command pattern:

```rust
#[tauri::command]
fn get_board_config(state: tauri::State<AppState>) -> Result<BoardConfig, String> {
    let path = state.project_dir.join("board.yaml");
    storage::read_board_config(&path)
}
```

### TypeScript / Vue

- Use `<script setup lang="ts">` in all Vue SFCs.
- Use Composition API only. No Options API.
- All application state lives in `useBoard.ts`. Components do not call `invoke()` directly. They call functions exposed by the composable.
- Use `ref()` and `computed()` from Vue. No `reactive()` for top-level state (it loses reactivity on destructure).
- Tauri IPC calls use `invoke` from `@tauri-apps/api/core`. Type the return values.

Example invoke pattern:

```typescript
import { invoke } from '@tauri-apps/api/core'
import type { Ticket } from '@/types'

async function getAllTickets(): Promise<Ticket[]> {
  return invoke<Ticket[]>('get_all_tickets')
}
```

### PrimeVue

- Import PrimeVue globally in `main.ts` with the Aura theme preset from `@primevue/themes`.
- Use PrimeVue components where they save effort: Button, Select (dropdowns), Dialog (modals), InputText, Tag, Toast, Textarea, ToggleButton, ConfirmDialog.
- Do not use PrimeVue's DataTable or layout components. The board layout is custom CSS.
- Import PrimeIcons in `main.ts`: `import 'primeicons/primeicons.css'`.
- Use PrimeIcons via class names: `<i class="pi pi-plus"></i>`.

PrimeVue setup in `main.ts`:

```typescript
import { createApp } from 'vue'
import PrimeVue from 'primevue/config'
import Aura from '@primevue/themes/aura'
import ToastService from 'primevue/toastservice'
import ConfirmationService from 'primevue/confirmationservice'
import 'primeicons/primeicons.css'
import App from './App.vue'

const app = createApp(App)
app.use(PrimeVue, {
  theme: {
    preset: Aura,
    options: {
      darkModeSelector: '.dark-mode'
    }
  }
})
app.use(ToastService)
app.use(ConfirmationService)
app.mount('#app')
```

### CSS

- Global styles in `src/styles/main.css`.
- Use CSS variables for theming (colors, spacing, sizes). Define in `:root` and override for dark mode.
- Component-specific styles use `<style scoped>` in the SFC.
- No Tailwind. No CSS-in-JS. Plain CSS only.
- PrimeVue's theme handles component styling. Custom CSS is only for layout and the board/card/lane UI.

### File naming

- Vue components: PascalCase (`TicketCard.vue`).
- TypeScript files: camelCase (`useBoard.ts`, `position.ts`).
- Rust files: snake_case (`storage.rs`, `models.rs`).

---

## 5. TypeScript types

Define these in `src/types/index.ts`. They must match the Rust structs. Field names are camelCase in TS (Rust uses snake_case but `#[serde(rename_all = "camelCase")]` handles the conversion).

```typescript
export interface BoardConfig {
  name: string
  prefix: string
  nextId: number
  lanes: string[]
  epics: Epic[]
  tags: string[]
  priorities: string[]
}

export interface Epic {
  id: string
  name: string
  color: string
}

export interface TicketMeta {
  id: string
  title: string
  status: string
  epic: string | null
  tags: string[]
  priority: string
  position: number
  created: string
}

export interface Ticket extends TicketMeta {
  body: string
}

export interface Index {
  generated: string
  ticketCount: number
  tickets: TicketMeta[]
}
```

---

## 6. Tauri configuration

### `tauri.conf.json` key settings

```json
{
  "productName": "Rojekti",
  "version": "0.1.0",
  "identifier": "com.rojekti.app",
  "build": {
    "frontendDist": "../dist",
    "devUrl": "http://localhost:1420",
    "beforeDevCommand": "npm run vite:dev",
    "beforeBuildCommand": "npm run vite:build"
  },
  "app": {
    "windows": [
      {
        "title": "Rojekti",
        "width": 1200,
        "height": 800,
        "minWidth": 600,
        "minHeight": 400
      }
    ],
    "security": {
      "csp": null
    }
  },
  "plugins": {
    "cli": {
      "description": "Local kanban board",
      "args": [],
      "subcommands": {
        "rebuild-index": {
          "description": "Rebuild index.yaml from ticket files"
        },
        "reorder": {
          "description": "Renormalize position values in a lane",
          "args": [
            {
              "name": "lane",
              "description": "Lane to reorder",
              "required": true,
              "takesValue": true
            }
          ]
        },
        "list": {
          "description": "List all tickets to stdout"
        },
        "show": {
          "description": "Print a single ticket to stdout",
          "args": [
            {
              "name": "id",
              "description": "Ticket ID",
              "required": true,
              "takesValue": true
            }
          ]
        },
        "create": {
          "description": "Create a new ticket",
          "args": [
            {
              "name": "title",
              "description": "Ticket title",
              "required": true,
              "takesValue": true
            },
            {
              "name": "status",
              "description": "Lane (defaults to first lane)",
              "takesValue": true
            },
            {
              "name": "priority",
              "description": "Priority level (defaults to medium)",
              "takesValue": true
            },
            {
              "name": "epic",
              "description": "Epic ID",
              "takesValue": true
            },
            {
              "name": "tags",
              "description": "Comma-separated tags",
              "takesValue": true
            }
          ]
        },
        "init": {
          "description": "Initialize a new board in the current directory",
          "args": [
            {
              "name": "name",
              "description": "Board name",
              "required": true,
              "takesValue": true
            },
            {
              "name": "prefix",
              "description": "Ticket ID prefix",
              "required": true,
              "takesValue": true
            }
          ]
        }
      }
    }
  }
}
```

Note: `beforeDevCommand` runs `npm run vite:dev` (just Vite, not Tauri). Tauri handles running the Rust side separately. The `dev` and `build` npm scripts are for the human/agent to invoke, and they call `npx tauri dev` / `npx tauri build` which orchestrate both Vite and Rust.

### Permissions

Tauri 2 uses a capability/permission system. In `src-tauri/capabilities/default.json`:

```json
{
  "identifier": "default",
  "description": "Default capability",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "cli:default"
  ]
}
```

The app does not use Tauri's filesystem plugin. All file I/O goes through custom Tauri commands backed by Rust's `std::fs`. This avoids scoping/permission issues entirely.

---

## 7. Markdown editor

### Library: md-editor-v3

GitHub: https://github.com/imzbf/md-editor-v3

A Vue 3 native markdown editor component. Ships two components:
- `MdEditor`: editor with toolbar (bold, italic, headings, lists, code, links, etc.).
- `MdPreview`: rendered markdown output.

### Installation

Already included in the dependency list above:

```bash
cmd.exe /c "npm install md-editor-v3"
```

### Usage in TicketModal

The description field in the ticket modal has two tabs: **View** (default) and **Edit**.

- **View tab**: uses `MdPreview` to render the ticket's markdown body as formatted HTML. Read-only.
- **Edit tab**: uses `MdEditor` with a toolbar for formatting. Outputs clean markdown.

```vue
<script setup lang="ts">
import { MdEditor, MdPreview } from 'md-editor-v3'
import 'md-editor-v3/lib/style.css'
</script>

<template>
  <div class="description-field">
    <div class="description-tabs">
      <button
        :class="{ active: descriptionTab === 'view' }"
        @click="descriptionTab = 'view'"
      >
        View
      </button>
      <button
        :class="{ active: descriptionTab === 'edit' }"
        @click="descriptionTab = 'edit'"
      >
        Edit
      </button>
    </div>

    <MdPreview
      v-if="descriptionTab === 'view'"
      :modelValue="ticket.body"
    />
    <MdEditor
      v-else
      v-model="ticket.body"
      :toolbars="editorToolbars"
      :preview="false"
    />
  </div>
</template>
```

### Toolbar configuration

Only include the formatting tools that make sense for ticket descriptions. No need for image upload, mermaid, katex, etc.

```typescript
const editorToolbars = [
  'bold',
  'italic',
  'strikeThrough',
  '-',
  'title',
  'unorderedList',
  'orderedList',
  'task',
  '-',
  'code',
  'codeRow',
  'link',
  '-',
  'revoke',
  'next'
]
```

The `-` entries add separator bars between toolbar groups.

### Theming

md-editor-v3 supports dark mode via the `theme` prop:

```vue
<MdEditor theme="dark" ... />
<MdPreview theme="dark" ... />
```

Set this based on the app's current theme (system preference detection). The composable should expose a `isDarkMode` ref that components can read.

### Important: no preview pane in editor

Set `:preview="false"` on `MdEditor`. This disables the built-in side-by-side preview. We do not want a split view inside the editor. The user switches between View and Edit tabs instead. The editor should be a full-width textarea with a toolbar on top.

---

## 8. Application state management

### AppState (Rust)

The Rust backend holds the project directory path in a managed state struct:

```rust
pub struct AppState {
    pub project_dir: std::path::PathBuf,
}
```

This is initialized in `main.rs` during `setup()` and passed to commands via `tauri::State<AppState>`.

### Project discovery (Rust, in main.rs setup)

```rust
fn discover_project_dir() -> Option<PathBuf> {
    // 1. Check current working directory
    if let Ok(cwd) = std::env::current_dir() {
        if cwd.join("board.yaml").exists() {
            return Some(cwd);
        }
    }
    // 2. Check directory containing the executable
    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            if exe_dir.join("board.yaml").exists() {
                return Some(exe_dir.to_path_buf());
            }
        }
    }
    None
}
```

If `discover_project_dir()` returns `None` in GUI mode, the frontend shows an initialization dialog. In CLI mode, `init` creates the board; other subcommands print an error and exit.

### useBoard composable (frontend)

This is the single source of truth for the frontend. Every component reads from and writes through this composable. It:

1. Exposes reactive state: `config`, `tickets`, `collapsedLanes`, `activeFilters`, `currentView`, `editingTicket`, `isDarkMode`.
2. Exposes computed properties: `ticketsByLane`, `ticketsByEpic`.
3. Exposes action functions: `loadBoard()`, `createTicket()`, `updateTicket()`, `deleteTicket()`, `moveTicket()`, etc.
4. Handles all `invoke()` calls internally.
5. Handles errors by pushing to PrimeVue's toast service.

Components never call `invoke()` directly. They call composable functions.

---

## 9. Implementation order

Follow this exact sequence. Complete each phase fully and verify it works before starting the next. Do not skip ahead.

### Phase 1: Rust foundation

Files to create/edit: `models.rs`, `storage.rs`, `index.rs`, `commands.rs`, `main.rs`

1. Define all Rust structs in `models.rs` with `#[serde(rename_all = "camelCase")]`.
2. Implement frontmatter parsing in `storage.rs`:
   - `parse_ticket_file(content: &str) -> Result<(TicketMeta, String), String>`
   - `serialize_ticket_file(meta: &TicketMeta, body: &str) -> String`
   - `read_board_config(path: &Path) -> Result<BoardConfig, String>`
   - `write_board_config(path: &Path, config: &BoardConfig) -> Result<(), String>`
   - `read_ticket(path: &Path) -> Result<Ticket, String>`
   - `write_ticket(dir: &Path, ticket: &Ticket) -> Result<(), String>`
   - `delete_ticket_file(dir: &Path, id: &str) -> Result<(), String>`
   - `list_ticket_files(dir: &Path) -> Result<Vec<PathBuf>, String>`
   - `read_all_tickets(dir: &Path) -> Result<Vec<Ticket>, String>`
3. Implement `rebuild_index` in `index.rs`.
4. Implement Tauri commands in `commands.rs`: `get_board_config`, `get_all_tickets`, `get_ticket`, `create_ticket`, `update_ticket`, `delete_ticket`, `move_ticket`, `reorder_lane`, `rebuild_index`, `init_project`.
5. Wire up `main.rs`: state management, command registration, project discovery.
6. **Verify**: create a test `board.yaml` and a few `.md` ticket files by hand. Run `npm run dev`. Open the browser console in the Tauri window. Call `window.__TAURI__.invoke('get_board_config')` and verify it returns config. Call `window.__TAURI__.invoke('get_all_tickets')` and verify it returns tickets. If either fails, fix before continuing.

### Phase 2: Frontend foundation

Files to create/edit: `types/index.ts`, `composables/useBoard.ts`, `App.vue`, `main.ts`, `styles/main.css`

1. Define TypeScript interfaces in `types/index.ts`.
2. Set up PrimeVue in `main.ts` (theme, toast service, icons).
3. Implement `useBoard.ts` composable with all state, computed properties, and action functions.
4. Implement `App.vue`: call `loadBoard()` on mount, show a loading state, render placeholder content.
5. **Verify**: the app loads, calls the Rust backend, and logs the board config + tickets to the console.

### Phase 3: Board UI

Files to create/edit: `TopBar.vue`, `BoardView.vue`, `Lane.vue`, `TicketCard.vue`

1. Build `TopBar.vue` with board name, view toggle buttons, filter dropdowns (PrimeVue Select), search input (PrimeVue InputText), new ticket button (PrimeVue Button with PrimeIcon).
2. Build `BoardView.vue` with flexbox lane layout.
3. Build `Lane.vue` with expanded/collapsed states, vertical name rotation, SortableJS initialization.
4. Build `TicketCard.vue` with priority border, epic badge, tag pills.
5. Wire drag-and-drop: `onEnd` handler calculates position, calls `moveTicket()` from the composable.
6. **Verify**: cards render in correct lanes. Drag a card between lanes, check that the `.md` file's frontmatter updated. Reorder within a lane, check position values updated. Collapse a lane, verify remaining lanes redistribute width.

### Phase 4: Ticket modal

Files to create/edit: `TicketModal.vue`

1. Build the modal using PrimeVue Dialog.
2. Create mode: form fields for title (InputText), status (Select), epic (Select), tags (toggleable pills), priority (Select).
3. Edit mode: same fields, pre-populated. View/Edit tabs for the description field.
4. Integrate md-editor-v3: `MdPreview` for view tab, `MdEditor` for edit tab. Configure toolbar (see section 7). Set `:preview="false"` on MdEditor.
5. Wire save (create/update), delete (with PrimeVue ConfirmDialog), and cancel.
6. **Verify**: create a ticket from the UI, see it appear on the board, check the `.md` file on disk. Edit it, toggle between View and Edit tabs for the description, save, verify file updated. Delete, verify file removed.

### Phase 5: Filtering and epic view

Files to create/edit: `filters.ts`, `EpicView.vue`, `EpicGroup.vue`

1. Implement filter logic in `filters.ts`. Export a function: `applyFilters(tickets, filters) -> filtered tickets`. Filters combine with AND logic.
2. Wire filter dropdowns in TopBar to `activeFilters` in the composable.
3. Build `EpicView.vue` and `EpicGroup.vue`.
4. Wire the view toggle.
5. **Verify**: filters hide/show cards correctly. Counts update. Epic view groups tickets properly. View toggle switches between board and epic views.

### Phase 6: CLI

Files to create/edit: `cli.rs`, update `main.rs`

1. Implement CLI dispatch in `main.rs` setup hook. If a subcommand is detected, handle it and call `std::process::exit(0)` before the GUI launches.
2. Implement all subcommand handlers in `cli.rs`, reusing `storage.rs` and `index.rs`.
3. **Verify**: build the app with `npm run build`. Find the built binary. Test:
   - `./rojekti rebuild-index` rebuilds `index.yaml`.
   - `./rojekti create --title "Test ticket"` creates a ticket `.md` file.
   - `./rojekti list` prints ticket summaries to stdout.
   - `./rojekti show --id TEST-001` prints a single ticket.
   - `./rojekti reorder --lane backlog` renormalizes positions.

During development, CLI args pass through with double dashes: `cmd.exe /c "npx tauri dev -- -- rebuild-index"`.

### Phase 7: Settings and polish

Files to create/edit: `BoardSettingsModal.vue`, update `main.css`

1. Build `BoardSettingsModal.vue` for editing lanes, epics, tags, priorities.
   - Lanes: ordered list with drag to reorder (SortableJS), add/delete. Deleting a lane with tickets prompts to move them first.
   - Epics: name input + color picker (`<input type="color">`). Add/edit/delete.
   - Tags: text input + enter to add, X to delete.
   - Priorities: ordered list. Add/edit/delete.
2. Dark/light theme based on system preference via `window.matchMedia('(prefers-color-scheme: dark)')`. Toggle `.dark-mode` class on `<html>` for PrimeVue and md-editor-v3 theming.
3. Keyboard shortcuts:
   - `Escape`: close any open modal.
   - `Ctrl+Enter` / `Cmd+Enter`: save in modal.
   - `N`: new ticket (when no modal is open).
4. Set window title to "Rojekti - {board name}" via Tauri's window API on load.
5. **Verify**: settings modal edits persist to `board.yaml`. Theme follows system preference. Keyboard shortcuts work.

---

## 10. Test data

Create a `test-data/` directory (gitignored) with a sample board for development. Copy it to the project root when testing.

### Sample `board.yaml`

```yaml
name: Test Board
prefix: TEST
next_id: 16
lanes:
  - backlog
  - todo
  - in-progress
  - review
  - done
epics:
  - id: auth
    name: Authentication
    color: "#E8A87C"
  - id: ui
    name: UI Overhaul
    color: "#85CDCA"
  - id: data
    name: Data Layer
    color: "#D4A5A5"
tags:
  - feature
  - bug
  - chore
  - research
priorities:
  - low
  - medium
  - high
  - critical
```

### Sample ticket (`tickets/TEST-001.md`)

```markdown
---
id: TEST-001
title: Implement login flow
status: in-progress
epic: auth
tags:
  - feature
priority: high
position: 1.0
created: 2026-03-10
---

Build the login form and hook it up to the auth backend.

## Requirements

- Email + password fields
- Form validation
- Error messages for invalid credentials
- Redirect to dashboard on success

## Notes

Use the existing API endpoint at `/api/auth/login`.
```

Create 10-15 tickets spread across different lanes, epics, tags, and priorities.

---

## 11. Common pitfalls

### Tauri 2 specifics

- Tauri 2 commands must be registered in `generate_handler![]` in `main.rs`. If you add a command but forget to register it, the frontend gets a silent error on invoke.
- Tauri 2 uses a capability/permission system. If a plugin feature does not work, check `src-tauri/capabilities/default.json`.
- The frontend dev URL is `http://localhost:1420` by default (Vite). Do not change this unless Vite's config also changes.

### serde_yaml

- `serde_yaml` serializes `None` as `null`. Use `#[serde(skip_serializing_if = "Option::is_none")]` on optional fields to omit them when empty.
- `serde_yaml` serializes empty `Vec` as `[]`. Use `#[serde(default, skip_serializing_if = "Vec::is_empty")]` to omit empty lists.
- When writing frontmatter back, the YAML field order may differ from the original. This is fine. The data is the same.

### Frontmatter parsing

- A ticket file starts with `---\n`, then YAML, then `---\n`, then the Markdown body.
- Use `splitn(3, "---")` to split. Element 0 is empty (before first `---`), element 1 is YAML, element 2 is body.
- Always trim the YAML and body after splitting.
- When writing back: `format!("---\n{}---\n\n{}\n", yaml, body)`. Ensure a blank line between closing `---` and body.

### SortableJS with Vue 3

- SortableJS manipulates the DOM directly, which can conflict with Vue's virtual DOM. After a drag operation, update the Vue state and let Vue re-render. Do not rely on SortableJS's DOM state as truth.
- Initialize SortableJS in `onMounted`. Destroy it in `onUnmounted` with `sortable.destroy()`.
- Use `data-` attributes on DOM elements for SortableJS to reference (ticket IDs, lane names). Do not parse component props from the DOM.
- When a lane collapses, destroy its SortableJS instance. Recreate when it expands.

### PrimeVue

- PrimeVue 4 uses a new theming system with design tokens. Import the theme preset (Aura) and pass it to the PrimeVue plugin config.
- PrimeVue components are tree-shakeable. Import only what you use: `import Button from 'primevue/button'`.
- For dark mode, PrimeVue uses a CSS class selector. Set `darkModeSelector: '.dark-mode'` in the theme options, then toggle the class on `<html>` based on system preference.

### md-editor-v3

- Import styles: `import 'md-editor-v3/lib/style.css'` in the component or globally.
- Dark mode: pass `theme="dark"` prop to both `MdEditor` and `MdPreview`.
- Disable the built-in preview pane with `:preview="false"`. We use a separate View tab instead.
- The editor emits clean markdown via `v-model`. No post-processing needed.
- Customize the toolbar via the `toolbars` prop (array of toolbar item names). See section 7.

### Position floats

- JavaScript floating point: `(1.0 + 2.0) / 2` is fine, but after many halvings, precision degrades. Trigger renormalization when `Math.abs(a - b) < 0.001`.
- Always sort by position numerically, never lexicographically.

### WSL2 / Windows path issues

- File paths in WSL2 use forward slashes (`/mnt/d/...`). Windows uses backslashes. When the agent edits files, use WSL2 paths. The `cmd.exe /c` delegation handles translation.
- Line endings: configure git to use LF in the repo. Add a `.gitattributes` file:
  ```
  * text=auto eol=lf
  ```
- If `npm run dev` fails with path-related errors, verify you are running from the project directory on the Windows filesystem, not from a WSL2-native path.

---

## 12. Do not

- Do not add dependencies not listed in this document without asking first.
- Do not use Pinia, Vuex, or any state management library.
- Do not use Vue Router.
- Do not use Tailwind CSS.
- Do not use the Tauri filesystem plugin. Use custom commands backed by `std::fs`.
- Do not use `unwrap()` or `expect()` in Rust command handlers.
- Do not use `any` in TypeScript.
- Do not create global mutable state in Rust outside of Tauri's managed state.
- Do not write tests yet. Manual testing only for v1.
- Do not implement features not described in `docs/PROJECT_PLAN.md`.
- Do not add comments explaining what code does. Only comment the "why" when it is not obvious.
- Do not use AI-typical filler in commit messages or code comments.
- Do not run `npm install` from WSL2 without going through `cmd.exe`. Use `cmd.exe /c "npm install <package>"` to ensure Windows-native node_modules.
- Do not place the project in the WSL2 home directory or any non-Windows filesystem path.