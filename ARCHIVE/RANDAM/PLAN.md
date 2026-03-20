# Rojekti — Project Plan

A local-first, repo-embedded project management tool built with Tauri 2. Designed to be used by humans through a kanban UI and by LLM agents through plain text files on disk.

---

## 0. Agent Build Instructions

This section is for the LLM agent that will implement this project. Read the full plan before writing any code.

### Environment

- **OS**: Windows 11 via WSL2 (Ubuntu).
- **Target platform**: Windows (`.exe` / `.msi`).
- **Tauri 2 on WSL2 for Windows targets**: Tauri's build toolchain needs native Windows tools. WSL2 cannot cross-compile to Windows natively. The recommended approach is:
  - Install Rust, Node, and npm **on the Windows side** (not inside WSL2).
  - Use PowerShell or Windows Terminal for `cargo tauri dev` and `cargo tauri build`.
  - The project source can live on the WSL2 filesystem, accessed from Windows via `\\wsl$\Ubuntu\...`, but build performance is better if the project lives on a native Windows path (e.g., `C:\dev\rojekti`).
  - Alternatively, install Rust and Node inside WSL2 and use WSLg for GUI rendering during development, then do release builds on the Windows side. This works but can be flaky with GPU acceleration.

### Prerequisites (Windows side)

1. **Rust**: install via `rustup` from https://rustup.rs. Ensure `cargo` is on PATH.
2. **Node.js 20+**: install via the official installer or `winget install OpenJS.NodeJS.LTS`.
3. **Tauri prerequisites**: Microsoft Visual Studio C++ Build Tools (installed via Visual Studio Installer, select "Desktop development with C++"). Also install WebView2 if not already present (ships with Windows 11).
4. **npm**: comes with Node.js.

### Scaffolding

```powershell
# From the Windows terminal
cd C:\dev
npm create tauri-app@latest rojekti -- --template vue-ts
cd rojekti
npm install
```

This creates the Tauri 2 + Vue 3 + TypeScript + Vite project scaffold.

Then install frontend dependencies:

```powershell
npm install sortablejs marked
npm install -D @types/sortablejs
```

The markdown editor (md-editor-v3) for the description field should be installed upfront with the other dependencies since it's a single package with no plugin decisions to make.

### Rust dependencies

In `src-tauri/Cargo.toml`, add:

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"
chrono = { version = "0.4", features = ["serde"] }
tauri-plugin-cli = "2"
```

### Build and run

```powershell
# Development (opens the app with hot reload on the frontend)
npx tauri dev

# Production build (outputs .exe and .msi installer)
npx tauri build
```

### Implementation approach

1. Read this entire plan before starting.
2. Follow the phases in section 12 in order. Each phase builds on the previous one.
3. Start with the Rust backend (models, file I/O, frontmatter parsing) before touching the frontend.
4. Get data flowing from filesystem to the Vue app before adding interactivity.
5. Use `invoke` from `@tauri-apps/api/core` for all frontend-to-backend communication.
6. Keep the frontend simple. No UI library. Plain CSS with the variables defined in section 8.3.
7. Test with real `.md` ticket files. Create a few by hand in `tickets/` to have sample data during development.
8. The CLI (Phase 5) shares all logic with the GUI commands. Don't duplicate code. Both call into the same `storage.rs` and `index.rs` modules.

### File naming convention

- Rust modules: `snake_case.rs`
- Vue components: `PascalCase.vue`
- Composables: `useBoard.ts`
- Utility files: `camelCase.ts`
- Ticket files: `{PREFIX}-{NNN}.md` (e.g., `ROJ-001.md`)

### What not to do

- Don't add a database. The filesystem is the database.
- Don't add a router. There's one view with a toggle (board/epics).
- Don't add Pinia or Vuex. Reactive refs in a composable are enough.
- Don't add Tailwind or any CSS framework. Plain CSS with variables.
- Don't over-engineer. This is a single-user local tool.

---

## 1. Project Overview

### Purpose

A lightweight issue tracker that lives inside a project repository. Tickets are stored as Markdown files with YAML frontmatter so LLM agents (Claude Code, Cursor, etc.) can read and edit them with standard file tools. A Tauri desktop app provides a graphical kanban interface for human use. The same binary also exposes a CLI for agents to trigger index rebuilds and other maintenance tasks without opening the GUI.

### Goals

- Zero friction to start: drop the binary into a project folder, run it, see the board.
- Data is always human-readable and LLM-editable plain text.
- No database, no server, no account, no sync service.
- Single-user, local-only. Not a collaboration tool.
- Multiple instances can run simultaneously for different projects.
- CLI access for LLM agents to perform maintenance operations.

### Non-Goals

- Multi-user or real-time collaboration.
- Cloud sync or remote access.
- Complex project management features (time tracking, sprints, burndown charts, resource allocation).
- Mobile support.

---

## 2. Architecture

### High-Level Overview

```
┌──────────────────────────────────────────────┐
│              Tauri 2 Shell                   │
│  ┌──────────────┐     ┌──────────────────┐   │
│  │  Rust Core   │     │   Vue 3 UI       │   │
│  │              │     │                  │   │
│  │ - File I/O   │     │ - Board view     │   │
│  │ - Frontmatter│     │ - Epic view      │   │
│  │   parse/write│     │ - Collapsible    │   │
│  │ - Index      │     │   lanes          │   │
│  │   rebuild    │     │ - Ticket modal   │   │
│  │ - CLI handler│     │ - WYSIWYG md     │   │
│  │              │     │   editor         │   │
│  └──────┬───────┘     └────────┬─────────┘   │
│         │      Tauri IPC       │             │
│         └──────────┬───────────┘             │
└────────────────────┼─────────────────────────┘
                     │
          ┌──────────▼───────────┐
          │   Local Filesystem   │
          │                      │
          │  board.yaml          │
          │  index.yaml          │
          │  tickets/            │
          │    ROJ-001.md        │
          │    ROJ-002.md        │
          │    ...               │
          └──────────────────────┘
```

### Component Responsibilities

**Rust backend (Tauri commands + CLI):**
- Read/write/delete ticket `.md` files in the `tickets/` directory.
- Parse YAML frontmatter and Markdown body from ticket files.
- Parse and serve `board.yaml` configuration.
- Rebuild `index.yaml` from ticket files on demand and on every write operation.
- Auto-generate ticket IDs using the prefix from `board.yaml` and an incrementing counter.
- Validate ticket data before writing.
- Handle CLI subcommands for headless operations (index rebuild, lane reorder, etc.).

**Vue 3 frontend:**
- Render the kanban board with collapsible lanes and draggable cards.
- Handle drag-and-drop reordering and lane changes.
- Provide a modal with a WYSIWYG Markdown editor for ticket descriptions.
- Render Markdown descriptions as formatted HTML in read mode.
- Provide an epic grouping view.
- Filter tickets by epic, tag, or priority.
- All state lives in Vue reactive refs. No external state management.

**Filesystem (source of truth):**
- `board.yaml` is the single config file for the board.
- Each ticket is one `.md` file with YAML frontmatter in `tickets/`.
- `index.yaml` is a derived, auto-rebuilt summary for LLM consumption.

---

## 3. Data Model

### 3.1 Board Configuration — `board.yaml`

```yaml
name: My Project Name (Placeholder)
prefix: ROJ
next_id: 44
lanes:
  - backlog
  - todo
  - in-progress
  - review
  - done
epics:
  - id: receipt-handling
    name: Receipt Handling
    color: "#E8A87C"
  - id: reporting
    name: Reporting
    color: "#85CDCA"
  - id: onboarding
    name: Onboarding
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

**Field definitions:**

| Field | Type | Description |
|---|---|---|
| `name` | string | Display name of the board. |
| `prefix` | string | Ticket ID prefix. Used to generate IDs like `ROJ-044`. |
| `next_id` | integer | Next auto-increment number for ticket IDs. Updated by the backend on ticket creation. |
| `lanes` | list of strings | Ordered list of lane identifiers. First lane is the default for new tickets. Order determines left-to-right display. |
| `epics` | list of objects | Each epic has `id` (slug), `name` (display), and `color` (hex). |
| `tags` | list of strings | Available tags for tickets. |
| `priorities` | list of strings | Ordered list of priority levels, lowest to highest. |

### 3.2 Ticket File — `tickets/{PREFIX}-{NNN}.md`

Example filename: `tickets/ROJ-042.md`

```markdown
---
id: ROJ-042
title: Add receipt OCR pipeline
status: backlog
epic: receipt-handling
tags:
  - feature
  - ocr
priority: high
position: 1.0
created: 2026-03-10
---

Parse uploaded receipt images using Tesseract. Extract vendor, total, date, and line items.

## Acceptance criteria

- Vendor name extracted correctly for 90% of test receipts.
- Total amount matches to the cent.
- Date parsed into ISO format.

## Notes

Consider using a preprocessing step to deskew and enhance contrast before OCR.
```

The YAML frontmatter contains all structured metadata. The Markdown body below the closing `---` is the ticket description. This format is:

- Natively renderable on GitHub and in any Markdown-aware editor.
- Easy for LLMs to parse (frontmatter is a well-known pattern from static site generators).
- The body supports full Markdown: headings, lists, code blocks, links, images, tables.
- No risk of YAML indentation issues in the description since it's outside the YAML block entirely.

**Frontmatter field definitions:**

| Field | Type | Required | Description |
|---|---|---|---|
| `id` | string | yes | Unique identifier. Format: `{prefix}-{zero-padded number}`. Generated by backend. |
| `title` | string | yes | Short summary. One line. |
| `status` | string | yes | Current lane. Must match a value in `board.yaml` lanes. |
| `epic` | string | no | Epic ID. Must match an epic `id` in `board.yaml`. Null/omitted if unassigned. |
| `tags` | list of strings | no | Zero or more tags. Must match values in `board.yaml` tags. |
| `priority` | string | yes | One of the values from `board.yaml` priorities. Default: `medium`. |
| `position` | float | yes | Sort order within a lane. Lower values appear higher/first. |
| `created` | date (ISO 8601) | yes | Creation date. Set by backend. Format: `YYYY-MM-DD`. |

**Markdown body:** Free-form description. Can be empty. Rendered as formatted HTML in the ticket modal. Edited via a WYSIWYG Markdown editor.

**Position value conventions:**

- New tickets get position `max_position_in_lane + 1.0`.
- Reordering between two tickets: new position = average of neighbors.
- If positions get too close (difference < 0.001), trigger a renormalization that reassigns positions as 1.0, 2.0, 3.0... for the entire lane.

### 3.3 Index File — `index.yaml`

Auto-generated. Never manually edited. Rebuilt by the backend on every write operation and on app startup. Can also be rebuilt via CLI.

```yaml
generated: 2026-03-16T14:30:00
ticket_count: 15
tickets:
  - id: ROJ-042
    title: Add receipt OCR pipeline
    status: backlog
    epic: receipt-handling
    tags:
      - feature
      - ocr
    priority: high
    position: 1.0
    created: 2026-03-10
  - id: ROJ-043
    title: Fix decimal rounding in totals
    status: in-progress
    epic: reporting
    tags:
      - bug
    priority: critical
    position: 2.0
    created: 2026-03-12
```

This file contains all frontmatter fields from every ticket but **not** the Markdown body. An LLM reads this once to understand the full board state. It only needs to open individual ticket files when it wants to read or edit the description.

---

## 4. Tauri Backend (Rust)

### 4.1 Dependencies

```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-cli = "2"
serde = { version = "1", features = ["derive"] }
serde_yaml = "0.9"
chrono = { version = "0.4", features = ["serde"] }
```

No external Markdown parser needed on the Rust side. The backend treats the Markdown body as an opaque string. Parsing and rendering happens in the frontend.

### 4.2 Frontmatter Parsing

The Rust backend parses ticket files by splitting on the `---` delimiters:

```rust
pub fn parse_ticket_file(content: &str) -> Result<(TicketMeta, String), String> {
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err("Invalid frontmatter format".into());
    }
    let meta: TicketMeta = serde_yaml::from_str(parts[1].trim())
        .map_err(|e| format!("YAML parse error: {}", e))?;
    let body = parts[2].trim().to_string();
    Ok((meta, body))
}

pub fn serialize_ticket_file(meta: &TicketMeta, body: &str) -> String {
    let yaml = serde_yaml::to_string(meta).unwrap();
    format!("---\n{}---\n\n{}\n", yaml, body)
}
```

### 4.3 Data Structures

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BoardConfig {
    pub name: String,
    pub prefix: String,
    pub next_id: u32,
    pub lanes: Vec<String>,
    pub epics: Vec<Epic>,
    pub tags: Vec<String>,
    pub priorities: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Epic {
    pub id: String,
    pub name: String,
    pub color: String,
}

/// Frontmatter metadata only (what gets parsed from/written to YAML)
#[derive(Serialize, Deserialize, Clone)]
pub struct TicketMeta {
    pub id: String,
    pub title: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epic: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub priority: String,
    pub position: f64,
    pub created: String,
}

/// Full ticket (frontmatter + body) sent to/from frontend
#[derive(Serialize, Deserialize, Clone)]
pub struct Ticket {
    #[serde(flatten)]
    pub meta: TicketMeta,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct Index {
    pub generated: String,
    pub ticket_count: usize,
    pub tickets: Vec<TicketMeta>,
}
```

### 4.4 Tauri Commands

All commands operate on the project directory, determined at startup by looking for `board.yaml` adjacent to the executable or in the current working directory (see section 6: Portable Binary).

#### `get_board_config`

```
Input:  none
Output: BoardConfig
```

Reads and parses `board.yaml` from the project directory.

#### `save_board_config`

```
Input:  BoardConfig
Output: ()
```

Writes updated config to `board.yaml`. Used when adding new lanes, epics, or tags from the UI.

#### `get_all_tickets`

```
Input:  none
Output: Vec<Ticket>
```

Reads every `.md` file in `tickets/`, parses frontmatter and body, returns sorted by lane then position.

#### `get_ticket`

```
Input:  id: String
Output: Ticket
```

Reads a single ticket file by ID. Returns both frontmatter and Markdown body.

#### `create_ticket`

```
Input:  title: String, status: String (optional, defaults to first lane),
        epic: Option<String>, tags: Vec<String>, priority: String,
        body: String
Output: Ticket
```

1. Read `board.yaml` to get current `next_id` and `prefix`.
2. Generate ID: `{prefix}-{next_id zero-padded to 3 digits}`.
3. Increment `next_id` in `board.yaml` and save.
4. Calculate position: highest position in target lane + 1.0.
5. Set `created` to today's date.
6. Write ticket file to `tickets/{id}.md` with frontmatter and body.
7. Rebuild index.
8. Return the created ticket.

#### `update_ticket`

```
Input:  Ticket (full object with meta + body)
Output: Ticket
```

Overwrites the ticket file with updated frontmatter and body. Rebuilds index.

#### `delete_ticket`

```
Input:  id: String
Output: ()
```

Deletes `tickets/{id}.md`. Rebuilds index.

#### `move_ticket`

```
Input:  id: String, new_status: String, new_position: f64
Output: Ticket
```

Reads the ticket file, updates `status` and `position` in the frontmatter, writes back. Body is preserved untouched. Rebuilds index. This is the command called during drag-and-drop.

#### `reorder_lane`

```
Input:  status: String, ticket_ids: Vec<String>
Output: ()
```

Takes the full ordered list of ticket IDs for a lane. Assigns positions 1.0, 2.0, 3.0... in order. Writes all affected ticket files (frontmatter only, body preserved). Rebuilds index. Called for renormalization.

#### `rebuild_index`

```
Input:  none
Output: Index
```

Scans `tickets/`, reads all files, extracts frontmatter, generates `index.yaml`. Also called internally after every write operation. Returns the rebuilt index.

### 4.5 Error Handling

All commands return `Result<T, String>` where the error string is a human-readable message. Common errors:

- File not found (ticket or board.yaml).
- Frontmatter parse error (corrupted YAML, likely from bad LLM edit).
- Invalid frontmatter format (missing `---` delimiters).
- Validation error (unknown lane, unknown epic, missing required field).
- Filesystem permission error.

The frontend displays errors as toast notifications.

---

## 5. CLI Mode

### 5.1 Purpose

The same Tauri binary doubles as a CLI tool. When invoked with subcommands, it performs operations headlessly (no window opens) and exits. This lets LLM agents trigger maintenance operations from the terminal.

### 5.2 Configuration

Uses the `tauri-plugin-cli` plugin. CLI is defined in `tauri.conf.json`:

```json
{
  "plugins": {
    "cli": {
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
          "description": "List all tickets (prints index to stdout)"
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

### 5.3 CLI Usage Examples

```powershell
# Initialize a new board in the current directory
./rojekti init --name "My Project" --prefix ROJ

# Rebuild the index after LLM edits
./rojekti rebuild-index

# Renormalize positions in a lane
./rojekti reorder --lane backlog

# List all tickets (outputs index.yaml content to stdout)
./rojekti list

# Show a specific ticket
./rojekti show --id ROJ-042

# Create a ticket from the command line
./rojekti create --title "Fix login bug" --priority high --tags "bug,auth"

# No subcommand = launch the GUI
./rojekti
```

### 5.4 CLI Implementation

In `main.rs`, check for CLI arguments before launching the window:

```rust
fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .setup(|app| {
            match app.cli().matches() {
                Ok(matches) => {
                    if let Some((subcommand, sub_matches)) = matches.subcommand {
                        handle_cli(subcommand, sub_matches);
                        std::process::exit(0);
                    }
                    // No subcommand: continue to GUI
                }
                Err(_) => {} // No CLI args: continue to GUI
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![...])
        .run(tauri::generate_context!())
        .expect("error running app");
}
```

CLI handlers reuse the same `storage.rs` and `index.rs` logic as the Tauri commands. No code duplication.

---

## 6. Portable Binary

### 6.1 Concept

The binary is designed to be dropped into any project directory. No installer, no global config, no folder picker dialogs.

### 6.2 Project Discovery

On startup (both GUI and CLI mode), the binary determines the project directory using this logic:

1. Check the **current working directory** for `board.yaml`.
2. If not found, check the **directory containing the executable** for `board.yaml`.
3. If found, use that directory as the project root.
4. If not found in either location:
   - **CLI mode with `init` subcommand**: create a new board in the current working directory.
   - **CLI mode without `init`**: print error and exit.
   - **GUI mode**: show an initialization dialog (enter board name and prefix, creates `board.yaml` and `tickets/` in the executable's directory).

### 6.3 Multiple Instances

Since each binary instance reads from its own adjacent directory, multiple instances can run simultaneously for different projects. Each gets its own window with its own board. No shared state, no conflicts.

Workflow:
1. Copy or symlink the binary into `project-a/rojekti.exe` and `project-b/rojekti.exe`.
2. Run both. Two windows open, each showing its own board.

### 6.4 Window Title

The window title shows the board name from `board.yaml` to distinguish instances: `"Rojekti - {board name}"`.

---

## 7. Frontend (Vue 3)

### 7.1 Dependencies

```json
{
  "dependencies": {
    "vue": "^3.5",
    "sortablejs": "^1.15",
    "md-editor-v3": "^5"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2",
    "@vitejs/plugin-vue": "^5",
    "vite": "^6"
  }
}
```

No Vue Router. No Pinia. No UI component library.

**md-editor-v3** is a Vue 3 markdown editor component with a built-in toolbar and preview. It supports edit-only mode, preview-only mode, and split view. The description field uses preview-only mode by default (rendered markdown) and switches to edit mode on a tab toggle. Markdown in, markdown out, no extra config.

### 7.2 Application State

All state lives in a single composable: `useBoard()`.

```typescript
// composables/useBoard.ts
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const config = ref<BoardConfig | null>(null)
const tickets = ref<Ticket[]>([])
const collapsedLanes = ref<Set<string>>(new Set())
const activeFilters = ref({
  epic: null as string | null,
  tag: null as string | null,
  priority: null as string | null,
  search: ''
})
const currentView = ref<'board' | 'epics'>('board')
const editingTicket = ref<Ticket | null>(null)
const isCreating = ref(false)

// Computed: tickets grouped by lane, respecting filters
const ticketsByLane = computed(() => {
  const filtered = applyFilters(tickets.value, activeFilters.value)
  const grouped: Record<string, Ticket[]> = {}
  for (const lane of config.value?.lanes ?? []) {
    grouped[lane] = filtered
      .filter(t => t.status === lane)
      .sort((a, b) => a.position - b.position)
  }
  return grouped
})

// Computed: tickets grouped by epic
const ticketsByEpic = computed(() => {
  const grouped: Record<string, Ticket[]> = { unassigned: [] }
  for (const epic of config.value?.epics ?? []) {
    grouped[epic.id] = []
  }
  for (const ticket of tickets.value) {
    const key = ticket.epic ?? 'unassigned'
    if (grouped[key]) {
      grouped[key].push(ticket)
    } else {
      grouped.unassigned.push(ticket)
    }
  }
  return grouped
})

// Lane collapse management
function toggleLaneCollapse(lane: string) {
  if (collapsedLanes.value.has(lane)) {
    collapsedLanes.value.delete(lane)
  } else {
    collapsedLanes.value.add(lane)
  }
}

function isLaneCollapsed(lane: string): boolean {
  return collapsedLanes.value.has(lane)
}
```

### 7.3 Component Tree

```
App.vue
├── TopBar.vue                     — Board name, view toggle, filters
├── BoardView.vue                  — Kanban board (default view)
│   └── Lane.vue (×N)             — Column (expanded or collapsed)
│       └── TicketCard.vue (×N)   — Draggable card
├── EpicView.vue                  — Tickets grouped by epic
│   └── EpicGroup.vue (×N)       — Single epic section
│       └── TicketCard.vue (×N)   — Same card component
├── TicketModal.vue               — Create/edit dialog with md editor
├── BoardSettingsModal.vue        — Edit lanes, epics, tags
└── Toast.vue                     — Notifications
```

### 7.4 Component Details

#### `App.vue`

Root component. On mount:
1. Call `get_board_config` to load the board.
2. If it fails (no `board.yaml` found), show the init dialog.
3. Call `get_all_tickets` to load all tickets.
4. Render the TopBar and the active view.

#### `TopBar.vue`

Horizontal bar at the top of the window.

Contents:
- Board name (from config).
- View toggle: "Board" | "Epics" (two buttons, highlights active).
- Filter dropdowns: Epic, Tag, Priority. Each is a `<select>` that updates `activeFilters`.
- Text search input: filters tickets by title substring match.
- "New Ticket" button: opens `TicketModal` in create mode.
- Settings gear icon: opens `BoardSettingsModal`.

#### `BoardView.vue`

The main kanban view. Renders one `Lane` per entry in `config.lanes`, arranged horizontally.

**Layout strategy for collapsible lanes:**

The board uses CSS flexbox. Each lane is a flex item:
- Expanded lanes: `flex: 1 1 0` (share available space equally).
- Collapsed lanes: `flex: 0 0 40px` (fixed narrow width).

This means expanded lanes automatically redistribute to fill the viewport when other lanes collapse, with no manual width calculation needed.

```css
.board {
  display: flex;
  gap: 12px;
  padding: 16px;
  height: calc(100vh - var(--topbar-height));
  overflow-x: hidden;
}

.lane {
  display: flex;
  flex-direction: column;
  min-width: 0;
  transition: flex 0.2s ease;
}

.lane.expanded {
  flex: 1 1 0;
  min-width: 200px;
}

.lane.collapsed {
  flex: 0 0 40px;
  cursor: pointer;
  overflow: hidden;
}
```

#### `Lane.vue`

A vertical column. Props: `lane` (string), `tickets` (array), `collapsed` (boolean).

**Expanded state:**
- Header: lane name (formatted: "in-progress" to "In Progress"), ticket count, collapse toggle button.
- Body: scrollable list of `TicketCard` components.
- SortableJS is initialized on the card container.

**Collapsed state:**
- Rendered as a narrow vertical strip (40px wide).
- Lane name displayed vertically, rotated 90 degrees.
- Ticket count shown below the name.
- Clicking anywhere on the collapsed lane expands it.
- No cards visible. No drag targets.

```html
<template>
  <div :class="['lane', collapsed ? 'collapsed' : 'expanded']">
    <!-- Collapsed view -->
    <div v-if="collapsed" class="lane-collapsed" @click="$emit('expand')">
      <span class="lane-name-vertical">{{ formatLaneName(lane) }}</span>
      <span class="lane-count">{{ tickets.length }}</span>
    </div>

    <!-- Expanded view -->
    <template v-else>
      <div class="lane-header">
        <h3>{{ formatLaneName(lane) }}</h3>
        <span class="lane-count">{{ tickets.length }}</span>
        <button class="collapse-btn" @click="$emit('collapse')">&#x2039;</button>
      </div>
      <div ref="cardContainer" class="lane-body" :data-lane="lane">
        <TicketCard
          v-for="ticket in tickets"
          :key="ticket.id"
          :ticket="ticket"
          @click="$emit('open-ticket', ticket)"
        />
      </div>
    </template>
  </div>
</template>
```

Collapsed lane name CSS:

```css
.lane-name-vertical {
  writing-mode: vertical-rl;
  text-orientation: mixed;
  transform: rotate(180deg);
  white-space: nowrap;
  font-weight: 600;
  font-size: 13px;
  letter-spacing: 0.05em;
  text-transform: uppercase;
}
```

**SortableJS config (expanded lanes only):**

```javascript
import Sortable from 'sortablejs'

onMounted(() => {
  if (!props.collapsed) {
    Sortable.create(cardContainer.value, {
      group: 'tickets',
      animation: 150,
      ghostClass: 'ghost',
      dragClass: 'dragging',
      onEnd: handleDragEnd
    })
  }
})
```

`handleDragEnd` extracts:
- The ticket ID from the dragged element's `data-ticket-id` attribute.
- The target lane from the drop container's `data-lane` attribute.
- The new index in the target lane.

Position calculation:
- If dropped at the start: `first_ticket_position - 1.0`.
- If dropped at the end: `last_ticket_position + 1.0`.
- If dropped between two tickets: `(prev_position + next_position) / 2`.
- If the lane is empty: `1.0`.

Calls `move_ticket` via Tauri invoke. Updates local state on success.

#### `TicketCard.vue`

A compact card representing one ticket.

Displays:
- Ticket ID (muted, small text, top-left).
- Title (main text, 1-2 lines, truncated with ellipsis if longer).
- Priority indicator (colored left border: critical=red, high=orange, medium=blue, low=gray).
- Epic badge (if assigned: small pill with epic name, colored with the epic's configured color).
- Tag pills (small, muted background, inline).

Click handler: emits `click` event with the ticket object. Parent sets `editingTicket` to open the modal.

The card element has `data-ticket-id` attribute for SortableJS to reference.

```html
<div
  class="ticket-card"
  :class="`priority-${ticket.priority}`"
  :data-ticket-id="ticket.id"
  @click="$emit('click', ticket)"
>
  <span class="ticket-id">{{ ticket.id }}</span>
  <h4 class="ticket-title">{{ ticket.title }}</h4>
  <div class="ticket-meta">
    <span
      v-if="ticket.epic"
      class="epic-badge"
      :style="{ backgroundColor: epicColor }"
    >
      {{ epicName }}
    </span>
    <span v-for="tag in ticket.tags" :key="tag" class="tag-pill">
      {{ tag }}
    </span>
  </div>
</div>
```

Card CSS for priority border:

```css
.ticket-card {
  border-left: 3px solid transparent;
  border-radius: 6px;
  padding: 10px 12px;
  margin-bottom: 8px;
  cursor: pointer;
  transition: box-shadow 0.15s;
}
.ticket-card:hover {
  box-shadow: 0 2px 8px rgba(0,0,0,0.12);
}
.priority-critical { border-left-color: #e53e3e; }
.priority-high     { border-left-color: #ed8936; }
.priority-medium   { border-left-color: #4299e1; }
.priority-low      { border-left-color: #a0aec0; }
```

#### `TicketModal.vue`

A centered modal dialog for creating or editing a ticket. Covers most of the viewport with a semi-transparent backdrop.

**Layout:**

```
+---------------------------------------------+
|  [ID]                            [X] Close  |
+---------------------------------------------+
|                                             |
|  Title: [________________________________]  |
|                                             |
|  Status: [dropdown]    Priority: [dropdown] |
|  Epic:   [dropdown]                         |
|  Tags:   [pill toggles]                     |
|                                             |
|  Created: 2026-03-10                        |
|                                             |
+---------------------------------------------+
|  Description                   [Edit|View]  |
| +-----------------------------------------+ |
| |                                         | |
| |  WYSIWYG Markdown editor                | |
| |  or rendered HTML preview               | |
| |                                         | |
| |                                         | |
| |                                         | |
| +-----------------------------------------+ |
|                                             |
|  [Delete]                     [Cancel][Save]|
+---------------------------------------------+
```

**Create mode** (no ticket passed):
- ID field is empty (auto-generated on save).
- Title input (text, required, autofocused).
- Status dropdown (defaults to first lane).
- Epic dropdown (optional, includes "None" option).
- Tags: clickable pills that toggle on/off (from `board.yaml` tags list).
- Priority dropdown.
- Description: md-editor-v3 in edit mode with toolbar, starts empty.
- "Create" and "Cancel" buttons.

**Edit mode** (ticket passed):
- All fields pre-populated.
- Description area has a View/Edit tab toggle.
  - **View** (default): md-editor-v3 in `previewOnly` mode. Rendered markdown, read-only.
  - **Edit**: md-editor-v3 in standard edit mode with toolbar.
- "Save", "Delete", and "Cancel" buttons.
- Delete button is styled as destructive (red) and positioned away from Save (left side).
- Delete shows a confirmation prompt before executing.

**Markdown editor integration:**

The description field uses md-editor-v3 (`MdEditor` component from `md-editor-v3`). Two modes controlled by a View/Edit tab pair:

- View tab sets `previewOnly` to true. The component renders markdown as formatted HTML.
- Edit tab sets `previewOnly` to false. The component shows the toolbar and editable textarea.
- The `modelValue` prop (v-model) binds to the ticket body string. Markdown in, markdown out.
- No additional rendering library needed. md-editor-v3 handles both editing and preview.

On save:
1. Validate required fields (title).
2. Collect frontmatter fields + Markdown body from the editor's v-model.
3. Call `create_ticket` or `update_ticket` via invoke.
4. Update local state with the returned ticket.
5. Close modal.

On delete:
1. Show confirmation: "Delete {ticket.id}? This cannot be undone."
2. If confirmed, call `delete_ticket` via invoke.
3. Remove from local state.
4. Close modal.

#### `EpicView.vue`

Alternative view toggled from TopBar.

Displays epics as horizontal sections (collapsible groups). Each section:
- Header: colored bar using epic color, epic name, ticket count.
- Body: tickets listed as `TicketCard` components (same card used in board view).
- Tickets within each epic are sorted by status (lane order) then position.

An "Unassigned" section at the bottom shows tickets with no epic.

In this view, tickets are not draggable between epics. To assign/unassign epics, users click the ticket to open the modal and change the epic dropdown.

#### `BoardSettingsModal.vue`

Modal for editing the board configuration.

Sections:
- **Board name**: text input.
- **Lanes**: ordered list with drag to reorder (SortableJS), add new lane button, delete button per lane. Deleting a lane that contains tickets shows a warning and requires the user to pick a target lane to move those tickets to first.
- **Epics**: list with name input, color picker (native `<input type="color">`). Add new, edit inline, delete. Deleting an epic unassigns all tickets from it.
- **Tags**: simple list. Add (text input + enter), delete (X button).
- **Priorities**: ordered list. Add, edit, delete. At least one must exist.

On save: calls `save_board_config`. Refreshes board state.

#### `Toast.vue`

Simple notification component. Displays messages at the bottom-right of the screen. Auto-dismisses after 3 seconds. Types: success (green), error (red), info (neutral).

Implemented as a reactive list of messages that components push to via the `useBoard` composable.

---

## 8. UI Design

### 8.1 Layout: Board View

```
+-------------------------------------------------------------+
| My Project         [Board|Epics]  [Epic v][Tag v][Pri v] [+] |
+------+---------------+---------------+---------------+------+
|#     |  Todo         | In Progress   |  Review       |#    |
|# B   |  (3)       [<]|  (2)       [<]|  (1)       [<]|# D  |
|# a   | +-----------+ | +-----------+ | +-----------+ |# o  |
|# c   | | ROJ-038   | | | ROJ-043   | | | ROJ-040   | |# n  |
|# k   | | Fix nav   | | | Rounding  | | | Export UI | |# e  |
|# l   | | * crit    | | | * crit    | | | o med     | |#    |
|# o   | | bug       | | | reporting | | | reporting | |# (7)|
|# g   | +-----------+ | +-----------+ | +-----------+ |#    |
|#     | +-----------+ | +-----------+ |               |#    |
|# (5) | | ROJ-041   | | | ROJ-045   | |               |#    |
|#     | | Settings  | | | OCR test  | |               |#    |
|#     | | o med     | | | o high    | |               |#    |
|#     | | chore     | | | receipt   | |               |#    |
|#     | +-----------+ | +-----------+ |               |#    |
|#     |               |               |               |#    |
+------+---------------+---------------+---------------+------+
```

In this example, "Backlog" and "Done" are collapsed (shown as narrow strips with `#`). The three expanded lanes share the remaining width equally. The collapsed lanes show their names vertically and ticket counts.

### 8.2 Visual Style

Minimal, clean, tool-like. Not flashy.

- **Theme**: follows system preference via `prefers-color-scheme`. Light and dark variants.
  - Light: white card background, light gray lane backgrounds, dark text.
  - Dark: dark gray card background, darker lane backgrounds, light text.
- **Cards**: slight elevation (1px border or subtle shadow), 6px border-radius.
- **Lanes**: subtle background tint to distinguish columns.
- **Typography**: system font stack (`-apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif`).
- **Priority colors**: critical = `#e53e3e`, high = `#ed8936`, medium = `#4299e1`, low = `#a0aec0`. Shown as a 3px left border on the card.
- **Epic badges**: use the epic's configured hex color as background. Text color auto-calculated (white for dark backgrounds, dark for light backgrounds) based on luminance.
- **Tag pills**: muted gray background, small font (12px), rounded.
- **Drag ghost**: 0.5 opacity clone of the card.
- **Drop placeholder**: dashed border outline (2px dashed, muted color) showing where the card will land.
- **Collapsed lanes**: darker background than expanded lanes, subtle visual separation.
- **Modal backdrop**: semi-transparent overlay (`rgba(0,0,0,0.5)`).

### 8.3 CSS Variables

```css
:root {
  --bg-primary: #ffffff;
  --bg-secondary: #f7f8fa;
  --bg-card: #ffffff;
  --bg-lane: #f0f2f5;
  --bg-collapsed: #e2e5ea;
  --text-primary: #1a1a2e;
  --text-secondary: #6b7280;
  --text-muted: #9ca3af;
  --border-color: #e5e7eb;
  --topbar-height: 52px;
  --lane-gap: 12px;
  --card-radius: 6px;
  --collapsed-width: 40px;
}

@media (prefers-color-scheme: dark) {
  :root {
    --bg-primary: #1a1a2e;
    --bg-secondary: #16213e;
    --bg-card: #1e293b;
    --bg-lane: #0f172a;
    --bg-collapsed: #1e293b;
    --text-primary: #e2e8f0;
    --text-secondary: #94a3b8;
    --text-muted: #64748b;
    --border-color: #334155;
  }
}
```

### 8.4 Window Behavior

Since this is a Tauri desktop app targeting Windows:

- **Minimum window size**: 600x400px.
- **Lanes**: minimum expanded width 200px. If the window is wide enough, lanes fill equally. If not, the user should collapse some lanes to make room. No horizontal scroll.
- **Modal**: max width 640px, centered, max height 80vh with scrollable body.
- **Card content**: title truncated to 2 lines with `text-overflow: ellipsis`. Tags truncated if they overflow one line.

---

## 9. LLM Agent Workflow

### 9.1 Reading the Board

An LLM agent working in the repo can understand the full project state with one file read:

```
Read file: index.yaml
```

This gives every ticket's ID, title, status, epic, tags, priority, position, and creation date. Enough to answer questions like "what's in progress?" or "what bugs are open?" without reading individual files.

### 9.2 Reading Ticket Details

To get the full description of a specific ticket:

```
Read file: tickets/ROJ-042.md
```

The file is standard frontmatter + Markdown. Every LLM understands this format natively.

### 9.3 Creating a Ticket

**Via CLI (preferred):**

```bash
./rojekti create --title "Fix login bug" --priority high --tags "bug,auth"
```

This handles ID generation, position assignment, and index rebuild automatically.

**Via direct file creation:**

1. Read `board.yaml` to get the current `next_id` and `prefix`.
2. Create `tickets/{prefix}-{next_id}.md` with frontmatter and body.
3. Increment `next_id` in `board.yaml` and save.
4. Run `./rojekti rebuild-index` to update the index.

### 9.4 Editing a Ticket

1. Read `tickets/ROJ-042.md`.
2. Modify the frontmatter fields or the Markdown body.
3. Write the file back.
4. Run `./rojekti rebuild-index`.

### 9.5 Moving a Ticket

1. Read `tickets/ROJ-042.md`.
2. Change `status` to the new lane name.
3. Set `position` to an appropriate value (read other tickets in that lane if needed, or just use a high number to append at the end).
4. Write the file back.
5. Run `./rojekti rebuild-index`.

### 9.6 Stale Index Handling

If the LLM edits tickets without rebuilding the index, `index.yaml` becomes stale. This is acceptable. The Tauri app rebuilds it on startup and after every GUI operation. The CLI `rebuild-index` command fixes it on demand. Staleness is never a data integrity issue. Individual ticket files are always the source of truth.

### 9.7 Agent Instructions Template

Projects using Rojekti can include instructions for LLM agents in their CLAUDE.md or equivalent:

```markdown
## Project Board

This project uses Rojekti for task tracking. Ticket files are in `tickets/`.

- Read `index.yaml` for a summary of all tickets.
- Each ticket is a `.md` file with YAML frontmatter in `tickets/`.
- After editing tickets, run `./rojekti rebuild-index` to update the index.
- To create a ticket: `./rojekti create --title "..." --priority medium`
- Lanes: backlog, todo, in-progress, review, done
- Priorities: low, medium, high, critical
```

---

## 10. Workflows

### 10.1 First Launch

1. Binary runs, looks for `board.yaml` in the working directory, then adjacent to the executable.
2. If found: load config and tickets, render the board.
3. If not found: show init dialog with fields for board name and ticket prefix.
4. On init: create `board.yaml` with defaults, create empty `tickets/` directory, create empty `index.yaml`.
5. Render the empty board.

### 10.2 Creating a Ticket (UI)

1. User clicks "+ New" in the TopBar.
2. `TicketModal` opens in create mode.
3. User fills in title (required), optionally sets status/epic/tags/priority.
4. User writes description in the WYSIWYG Markdown editor.
5. User clicks "Create".
6. Frontend calls `create_ticket` command.
7. Backend generates ID, writes `.md` file, rebuilds index, returns ticket.
8. Frontend adds ticket to local state. Card appears in the appropriate lane.
9. Modal closes. Toast confirms creation.

### 10.3 Editing a Ticket (UI)

1. User clicks a card.
2. `TicketModal` opens in edit mode with ticket data.
3. Description area shows rendered Markdown (view mode) by default.
4. User clicks "Edit" toggle to switch to WYSIWYG editor.
5. User modifies fields and/or description.
6. User clicks "Save".
7. Frontend calls `update_ticket` with the full ticket object (meta + body).
8. Backend writes `.md` file with updated frontmatter and body, rebuilds index.
9. Frontend updates local state. Card re-renders.

### 10.4 Dragging a Ticket Between Lanes

1. User drags a card from "Backlog" to "In Progress".
2. SortableJS fires `onEnd` event.
3. Handler calculates new position based on drop index among target lane's cards.
4. Frontend calls `move_ticket` with ticket ID, new status, new position.
5. Backend reads the ticket file, updates frontmatter (`status` and `position`), preserves the Markdown body, writes back, rebuilds index.
6. Frontend updates local state. Card appears in new lane at correct position.

### 10.5 Dragging to Reorder Within a Lane

Same as cross-lane drag, but `status` stays the same. Only `position` changes.

### 10.6 Collapsing/Expanding Lanes

1. User clicks the collapse button on a lane header.
2. Lane transitions to collapsed state (narrow strip, vertical name).
3. Remaining expanded lanes redistribute width via flexbox.
4. Collapsed lane's tickets are hidden (still in state, just not rendered).
5. Clicking the collapsed lane strip expands it.
6. Collapse state is stored in `collapsedLanes` ref (in-memory only, resets on reload).

### 10.7 Filtering

1. User selects an epic/tag/priority from the filter dropdowns.
2. `activeFilters` ref updates.
3. `ticketsByLane` recomputes, hiding non-matching tickets.
4. Cards that don't match disappear. Lane ticket counts update.
5. Clearing a filter (selecting "All" or empty option) restores all tickets.
6. Text search filters by case-insensitive substring match on ticket title.
7. Filters combine with AND logic (matching epic AND tag AND priority AND search).

### 10.8 Switching to Epic View

1. User clicks "Epics" in the TopBar.
2. `currentView` changes to `'epics'`.
3. `EpicView` renders with tickets grouped by epic.
4. Clicking a card still opens the modal.
5. User can change epic assignment in the modal's epic dropdown.
6. Clicking "Board" switches back.

### 10.9 Deleting a Ticket

1. User opens a ticket in the modal.
2. Clicks "Delete" (red button, bottom-left).
3. Confirmation dialog: "Delete ROJ-042? This cannot be undone."
4. User confirms.
5. Frontend calls `delete_ticket`.
6. Backend deletes the `.md` file, rebuilds index.
7. Frontend removes from state. Card disappears.
8. Modal closes. Toast confirms deletion.

---

## 11. Project File Structure

### Application Source

```
rojekti/
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── src/
│       ├── main.rs              -- Entry point, CLI dispatch or GUI launch
│       ├── cli.rs               -- CLI subcommand handlers
│       ├── commands.rs          -- Tauri GUI command handlers
│       ├── models.rs            -- Struct definitions
│       ├── storage.rs           -- File I/O (read/write frontmatter+md, scan dir)
│       └── index.rs             -- Index rebuild logic
├── src/
│   ├── main.ts                  -- Vue app entry point
│   ├── App.vue                  -- Root component
│   ├── composables/
│   │   └── useBoard.ts          -- All state and Tauri invoke wrappers
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
│   │   ├── position.ts          -- Position calculation helpers
│   │   └── filters.ts           -- Ticket filtering logic
│   └── styles/
│       └── main.css             -- Global styles, CSS variables, theme
├── index.html
├── vite.config.ts
├── package.json
└── tsconfig.json
```

### User's Project (what gets created in the target repo)

```
my-project/
├── rojekti.exe                  -- The binary (copied here)
├── board.yaml                   -- Board configuration
├── index.yaml                   -- Auto-generated ticket summary
└── tickets/
    ├── ROJ-001.md
    ├── ROJ-002.md
    ├── ROJ-003.md
    └── ...
```

---

## 12. Implementation Order

### Phase 1: Foundation

1. Scaffold Tauri 2 + Vue 3 + Vite project using `npm create tauri-app@latest`.
2. Define Rust data structs in `models.rs`.
3. Implement frontmatter parsing (split on `---`, parse YAML, extract body).
4. Implement file I/O helpers in `storage.rs`: read/write `.md` ticket files, scan `tickets/` directory.
5. Implement `init_project` (create `board.yaml`, `tickets/`, `index.yaml`).
6. Implement `get_board_config`, `get_all_tickets`, `get_ticket`.
7. Implement `create_ticket` with ID generation.
8. Implement `rebuild_index`.
9. Basic frontend: render a static board with data from Tauri commands.

### Phase 2: Board UI

1. Implement `BoardView` and `Lane` components with flexbox layout.
2. Render real data from Tauri commands into lanes and cards.
3. Implement `TicketCard` with priority border, epic badge, tag pills.
4. Add SortableJS to expanded lanes.
5. Implement `move_ticket` command.
6. Wire drag-and-drop to `move_ticket` with position calculation.
7. Implement lane collapsing with CSS transitions and vertical name rendering.

### Phase 3: Ticket Management

1. Implement `TicketModal` in create mode (form fields + md-editor-v3 for description in edit mode).
2. Implement edit mode with View/Edit tab toggle: View tab shows md-editor-v3 in `previewOnly` mode, Edit tab shows the full editor with toolbar.
3. Implement `update_ticket` and `delete_ticket` commands.
4. Wire modal save/delete to commands.
5. Add toast notifications for feedback.

### Phase 4: Filtering and Epic View

1. Implement filter dropdowns in `TopBar`.
2. Implement combined filter logic (AND) in `useBoard` composable.
3. Implement text search filter.
4. Build `EpicView` and `EpicGroup` components.
5. Wire view toggle between board and epic views.

### Phase 5: CLI Mode

1. Add `tauri-plugin-cli` dependency.
2. Define CLI subcommands in `tauri.conf.json`.
3. Implement CLI dispatch in `main.rs` (detect subcommands, handle before GUI launch).
4. Implement CLI handlers in `cli.rs`, reusing `storage.rs` and `index.rs` logic.
5. Test: `rebuild-index`, `reorder`, `list`, `show`, `create`, `init`.

### Phase 6: Settings and Polish

1. Implement `BoardSettingsModal` (edit lanes, epics, tags, priorities).
2. Implement `save_board_config`.
3. Dark/light theme via CSS variables and `prefers-color-scheme`.
4. Keyboard shortcuts:
   - `Escape`: close modal.
   - `Ctrl+Enter`: save modal.
   - `N`: new ticket (when no modal is open).
5. Window title: "Rojekti - {board name}".
6. Project discovery logic (check cwd then exe directory for `board.yaml`).

---

## 13. Future Considerations

Not in scope for v1, but designed around to avoid blocking later:

- **Archiving**: move "done" tickets to an `archive/` directory after a configurable age. The index excludes archived tickets. A separate CLI command or UI button triggers archiving.
- **Bulk operations**: multi-select cards (checkboxes), bulk move/tag/assign epic.
- **Due dates**: optional `due` field in frontmatter. Cards show overdue indicator.
- **Ticket linking**: `blocked_by` and `blocks` fields as lists of ticket IDs. Visual indicators on cards.
- **Activity log**: append-only `changelog.yaml` recording ticket changes with timestamps.
- **File watcher**: auto-reload the UI when ticket files change on disk (e.g., after an LLM edits them). Use `notify` crate in Rust.
- **Subtasks**: checklist items within a ticket, tracked in the Markdown body as `- [ ]` / `- [x]` items.
- **CLI output formats**: `--format json` flag for CLI commands so agents can parse output programmatically.
- **Linux/macOS builds**: currently targeting Windows only. The architecture is cross-platform by design. Adding other targets is a build config change, not a code change.