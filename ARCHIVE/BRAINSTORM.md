# Rojekti: Ideas & Future Direction

This document captures feature ideas and vision for Rojekti. Each idea has been reviewed against
the core goal: a **single-user, local-first, CLI+GUI kanban with a rock-solid foundation**. Not
everything belongs in the app. The bar is: does this make the core workflow meaningfully better,
or is it scope creep?

> **What should a single-user desktop kanban do better than Trello?**
> - Offline-first by design — no connectivity, no vendor, no subscription.
> - Files are the database — cards can be git-committed, diffed, scripted, read by agents.
> - CLI-accessible — full board manipulation without opening a GUI.
> - Agent-friendly — AI can read and write cards directly via filesystem.
> - Speed — everything is local, no API round trips.
> - No noise — single user means no comments, @mentions, activity feeds, or collaboration
>   overhead. Every feature should serve the solo operator.

---

## 🏎️ Usability & Power-User Workflows

### 1. Command Palette (`Ctrl+K`)
A centralized hub for navigating the app without a mouse. Inspired by VS Code and Discord.
- **Quick Jump**: Type an ID like `ROJ-042` to open that card immediately.
- **Action Execution**: Type commands like `new`, `settings`, `archive all done`.
- **Search**: Fuzzy search across titles, descriptions, and tags in one interface.

> **✅ Strong yes — implement this.**
> This is the single highest-leverage usability feature. Once a board has 30+ cards, mouse-only
> navigation becomes slow. A command palette makes every feature instantly accessible.
> Implementation: fuzzy filter over `cards.value` titles + IDs on the frontend, a few hardcoded
> action keywords. No new backend needed. This is something Trello has only partially solved and
> a desktop-native app can do far better.

---

### 2. Global "Capture" Hotkey (`Ctrl+Shift+N`)
A background listener that spawns a small "Quick Entry" window even when the app is minimized.
- **Use Case**: You're in a browser or IDE and have an idea. Hit the hotkey, type the title, hit
  Enter. The window closes and the card lands in the Backlog.
- **Backend**: Requires `tauri-plugin-global-shortcut`.

> **⚠️ Good idea, medium complexity — defer until foundation is solid.**
> The use case is real and valuable. The risk is OS-level global shortcut registration, which has
> cross-platform edge cases (conflicts with other apps, behavior when app is fully closed vs
> minimized). `tauri-plugin-global-shortcut` is a maintained official plugin so it should be
> safer than the opener situation. Worth doing after the core UX is stable.
> A simpler first pass: a quick-add bar always visible at the top of the board (no global hotkey,
> just a focused text field that creates a backlog card on Enter).

---

### 3. Shift-Select & Bulk Operations
Hold Shift and click multiple cards. A selection bar appears with batch actions.
- **Batch Edit**: Change Status, Epic, Priority, or add a Tag to all selected cards at once.
- **Move to Top/Bottom**: Re-sort a group of cards instantly.

> **✅ Yes — practical and underrated.**
> Most kanban tools handle this poorly. "Move all these 8 done cards to archive" or "tag all
> these with 'sprint-3'" are real daily workflows. Implementation is self-contained in the
> frontend (a `selectedCards: Set<string>` ref, a selection toolbar component, and calling
> existing `updateCard`/`moveCard` in a loop). No backend changes needed.

---

## 📊 Visualization & Advanced Organization

### 1. Horizontal Swimlanes
Columns represent Status, rows represent Epics or Priority — a 2D grid layout.

> **❌ Skip for now.**
> Swimlanes look impressive in demos but real single-user workflows rarely need them. The Epic
> view already provides a different grouping lens. A 2D grid is a significant UI rewrite
> (responsive layout, drag-and-drop across two axes, SortableJS complications). Jira has
> swimlanes and most people leave them disabled. The complexity cost is too high for the benefit.

---

### 2. Card Aging & Stagnation Detection
Visual signal for neglected cards based on `last_modified` timestamp in frontmatter.
- A "cobweb" icon or reduced opacity appears after N days without movement.
- A "Stagnant" filter shows only cards untouched for 2+ weeks.

> **✅ Yes — low effort, high signal value.**
> This requires adding `last_modified` to the card frontmatter (updated on every write in
> `commands.rs`). The frontend can then compute staleness purely from that field. No new backend
> commands needed. A subtle visual cue (e.g. a small clock icon or muted card border) is enough
> — no need for opacity changes which can feel heavy-handed. The stagnant filter is a one-liner
> addition to the filter logic in `useBoard.ts`. This is something Trello doesn't surface at all
> and is genuinely useful for catching forgotten work.

---

### 3. Work-In-Progress (WIP) Limits
Set a max card count per status lane in board settings.
- If exceeded, the column header turns red and the add button is disabled.

> **✅ Yes — core Kanban discipline, easy to implement.**
> WIP limits are the discipline that makes Kanban actually work instead of just being a visual
> to-do list. Implementation: optional `wip_limit` field on the `Status` struct (already has
> `id` and `name`). Frontend checks `cardsByStatus[status.id].length >= status.wipLimit` and
> applies a warning class to the lane header. This is a config field + a CSS class — minimal
> complexity. Default to no limit so it's opt-in.

---

## 🛠️ Content & Card Metadata

### 1. GFM Checklists / Sub-task Progress
Detect `- [ ]` and `- [x]` patterns in the Markdown body.
- Show a progress bar on the card face: `[████░░] 4/6`.
- Clicking a checkbox on the card face updates the Markdown file directly.

> **✅ Strong yes — two-phase implementation.**
> Phase 1 (easy): Parse the card body in the frontend for checkbox patterns and render a
> progress bar on the card face. Pure frontend, read-only, zero backend changes.
> Phase 2 (harder): Make checkboxes on the card face clickable — requires a backend command that
> patches the specific line in the Markdown body. Useful but can be deferred.
> This turns cards into mini-specs with visible completion state, which is exactly the right
> scope for a single-user tool.

---

### 2. Dynamic Custom Fields
Define extra frontmatter fields per board in `rojekti.config.yaml`.
- Examples: `estimate_hrs` (Number), `reviewer` (String), `due_date` (Date).

> **❌ Overengineered for single-user — skip.**
> The free-form Markdown body already handles ad-hoc metadata cleanly. Custom fields require a
> schema definition system, typed input rendering per field type, serde deserialization for
> arbitrary fields, and CLI awareness of those fields. The upside for a single user is minimal —
> you can just write `**Estimate:** 3h` in the body. If a `due_date` field specifically becomes
> a pattern that many features depend on (card aging, calendar view), add it as a first-class
> field then — not as a generic system.

---

### 3. Card Relations & Dependencies
Link cards by ID. Show a "Blocked by ROJ-100" badge on card faces.
- Parent/child breakdown for larger features.

> **⚠️ Blocked-by yes, parent/child no.**
> A `blocked_by: [ROJ-100]` field in frontmatter is trivial to add and genuinely useful — you
> can see at a glance that a card can't progress without reading its description. The badge on
> the card face requires no backend changes, just frontend parsing of a frontmatter field.
> Parent/child hierarchies add significant UI complexity (indented sub-cards, rollup status,
> nested drag-and-drop) for a single-user tool where a checklist in the card body does the same
> job more simply.

---

## 📁 System Integration & Ecosystem

### 1. Native File Attachments
Already specced in `ATTACHMENTS_SPEC.md`. Drag files onto a card, copied or linked. Images
render as thumbnails, documents open in default app.

> **✅ In progress — full spec exists.**

---

### 2. Git Integration
If Rojekti is in a Git repo, detect branches matching Card IDs (e.g. `feature/ROJ-001`).
- Show a branch badge on the card face.
- Stretch: auto-move card to "Review" when a PR is detected.

> **⚠️ Branch detection yes, PR sync no.**
> Running `git branch --list "*/ROJ-*"` via `std::process::Command` is cheap, safe, and
> entirely local. A small branch badge on the card face ("🔀 feature/ROJ-001") gives useful
> context with near-zero complexity — just a new optional Tauri command that returns branch
> names for a given card ID prefix.
> PR status sync requires GitHub/GitLab API credentials, polling or webhooks, error handling for
> rate limits, and offline degradation. It undermines the offline-first principle. Skip.

---

### 3. Archive (distinct from soft-delete)
A state where cards are hidden from the board but not deleted.
- `archived: true` flag in frontmatter. Cards stay in `rojekti/cards/`.
- An "Archived" view to browse and restore them.

> **✅ Yes — important distinction from delete.**
> "Done" cards from 6 months ago shouldn't clog the board but also shouldn't be in the deleted
> folder. Archive is the right middle ground. Implementation: add `archived: bool` to
> `CardMeta`, filter out archived cards in `get_all_cards` by default, add a toggle to show
> them. Cards can be archived via a card menu or via CLI. This is much simpler than it sounds —
> it's a single boolean field and a filter.

---

## 🎨 Aesthetic & Sensory Experience

### 1. Layout Transitions
`<TransitionGroup>` for cards — smooth slides when filtering, `fade` on view switches.

> **⚠️ Partial yes — filtering only.**
> Fade transitions when applying/removing filters are clean and straightforward with Vue's
> `<TransitionGroup>`. Avoid animating card movement during drag — SortableJS manages the DOM
> directly during drag events and Vue transitions will fight it, causing visual glitches
> (we already know SortableJS/Vue DOM conflicts are a pitfall in this project). Scope this to
> filter changes and view transitions only.

---

### 2. Tactile Audio Feedback
Subtle sounds on card drop, task completion.

> **❌ Not for this app.**
> Sound in a productivity tool is almost always a net negative for professional users. It's
> distracting in shared spaces and condescending after the first hour. If ever added, it must
> be opt-in and off by default, which means the feature exists mostly to be turned off. Not
> worth the implementation time.

---

## 🤖 Agent-Friendly Features

### 1. `agent_context` Frontmatter Field
A dedicated space in frontmatter for AI instructions.
- `agent_context: "Research the payments API and add a technical plan to this card."`
- Agents can filter for cards with pending `agent_context` via the CLI.

> **✅ Zero-cost, high-value — add it.**
> This is the point of the whole project for AI/human workflows. It's literally just a new
> optional text field in `CardMeta` with `#[serde(skip_serializing_if = "Option::is_none")]`.
> No UI changes strictly needed — it's set via CLI or by directly editing the card file. The
> CLI could get a `list --has-agent-context` filter. The card modal could show a distinct
> "Agent Instructions" text area if the field is present. Add it now — it costs nothing.

---

### 2. History & Audit Log (`rojekti.history.jsonl`)
A JSONL log of all board actions: `{ timestamp, action, card_id, from, to, actor }`.
- Enables undo and gives agents recent context on board activity.

> **⚠️ Log yes, undo UI no — for now.**
> Writing a JSONL entry to `rojekti.history.jsonl` on every state-changing command is trivial
> and provides immediate value for agents (they can read recent history to understand what's
> been happening). The file stays human-readable and machine-parseable.
> Full undo (reversing arbitrary past actions) requires reconstructing state from the log, which
> is a significant correctness problem — especially with external edits in the mix. Log the
> events now. Build undo later if the demand is clear.
> Note: `rojekti.history.jsonl` should be added to the file watcher's ignore list, same as
> `rojekti.index.yaml`.
