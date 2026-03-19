# Rojekti Backlog

## 🟢 Easy Wins
*High impact, low technical risk. Good for immediate polish.*

- [ ] **Graceful handling of broken card files**
  If any card file has malformed YAML frontmatter, `read_all_cards` in `storage.rs` fails entirely via `?` — the whole board disappears with no explanation. Fix: change `read_all_cards` to collect errors per file rather than short-circuit. Broken cards should be skipped and a warning toast shown per bad file (e.g. "Could not load ROJ-007.md: YAML parse error"). The rest of the board loads normally. This is important because users can hand-edit card files via CLI or a text editor and introduce typos.

- [ ] **Timestamp suffix in deleted folder to prevent overwrites**
  `delete_card_file` in `storage.rs` uses `fs::rename`, so a delete followed by restore followed by delete again is safe — the restore moves the file back out of `deleted/`. However, if a card file is manually copied back into `rojekti/cards/` (rather than moved) while the original still sits in `rojekti/deleted/`, a subsequent deletion would overwrite the deleted copy. Fix: append a timestamp to the destination filename — e.g. `ROJ-001_2026-03-19T142301.md` — to make collisions impossible regardless of how restore is implemented.

- [ ] **Validate board name and prefix against OS-reserved names**
  On Windows, filenames like `CON.md`, `PRN.md`, `AUX.md` etc. are reserved device names and will fail or behave unpredictably. The card prefix becomes part of filenames (`PREFIX-001.md`), so a prefix of `CON` or `NUL` is dangerous. Fix: add a blocklist check in `init_project` in `commands.rs` that rejects reserved Windows names (`CON`, `PRN`, `AUX`, `NUL`, `COM1`–`COM9`, `LPT1`–`LPT9`) case-insensitively for the prefix field, and strips illegal path characters (`\ / : * ? " < > |`) from both name and prefix. Return a descriptive error if validation fails.

- [ ] **Clear stale filters when Epic/Tag is deleted**
  If a user has an active Epic or Tag filter and then deletes that Epic/Tag in Settings, the board appears completely empty with no explanation. Filters referencing deleted items should be cleared automatically when `saveBoardConfig` is called.
  Fix in `useBoard.ts`: after `config.value` is updated by `saveBoardConfig`, check `activeFilters.value.epic` and `activeFilters.value.tag` against the new config's epics/tags arrays. If the filtered ID no longer exists, reset that filter to `null`.

- [ ] **Verify CardModal does not silently drop card fields on save**
  Code review flagged a risk: if `CardModal.vue` uses a partial/incomplete local state object for editing and spreads it onto the saved card, any fields present on the original card but absent in the local ref will be silently lost on save. Read `CardModal.vue` and confirm the save path works with a complete `Card` object. If not, fix the local state type to be a strict full `Card`.

- [ ] **Verify TopBar.vue filter popover ref is correctly wired**
  Code review flagged that `const filterPanel = ref()` may not be correctly linked to `<Popover ref="filterPanel">` in all configurations. Read `TopBar.vue` and confirm the ref assignment works and the popover opens/closes correctly.

---

## 🔴 Serious Work
*Requires deeper logic changes, debugging, or new dependencies.*

- [ ] **Atomic file writes to prevent corruption**
  `storage::write_card` and `storage::write_board_config` both use `fs::write` which is not atomic. If the process crashes mid-write the file is left truncated or corrupted with no recovery path.
  Fix: write to a sibling `.tmp` file first, then `fs::rename` to the target. Rename is atomic on all target platforms. Applies to both `write_card` and `write_board_config` in `storage.rs`. The `.tmp` extension is already filtered by the file watcher so no watcher side-effects.

- [ ] **Silent background refresh for watcher-triggered reloads**
  `loadBoard()` sets `isLoading = true` which renders a full-screen `ProgressSpinner` overlay (`position: fixed; z-index: 1000`). When the file watcher triggers a reload this causes a jarring full-screen flash.
  Fix: add a `silent` boolean parameter to `loadBoard`. When `true`, skip setting `isLoading` and omit the spinner — just silently swap `config.value` and `cards.value` in the background. The `board-changed` listener in `App.vue` calls the silent variant. The initial `onMounted` load and `initBoard` continue using the normal variant.

- [ ] **Card attachments**
  Full spec in `ATTACHMENTS_SPEC.md`. Cards can have files attached — images render as thumbnails inline, audio/video with HTML5 players, all other formats open in the OS default app. Files are either copied to `rojekti/attachments/` or linked by path depending on a global board setting. Local attachments are moved to `rojekti/attachments/deleted/` when their card is deleted.

- [ ] **Internationalization (Translations)**
  The app has hardcoded English strings. Low priority until core features are stable. When the time comes, extract strings into `vue-i18n` and replace hardcoded text with keys. No target languages defined yet — needs clarification before starting.

---

## 🔍 Needs Clarification
*Requires user input or verification before proceeding.*

- [ ] **Collapsed lane card count rotation**
  When a lane is collapsed, the lane name rotates correctly but the card count number does not. The count should sit above the lane name (toward the top of the lane) and also rotate with it.
  Needs visual confirmation of current state before writing the CSS fix.

---

## ✅ Completed

- [x] **Rename "Tickets" to "Cards"** — Bulk search-and-replace across Rust models, Vue components, TypeScript interfaces, and `cards/` directory rename.

- [x] **Fix Markdown Editor width mismatch** — Normalized internal padding and borders for `md-editor-v3` so Edit and View tabs align.

- [x] **Default window size and responsiveness** — Adjusted CSS flex logic and `tauri.conf.json` window dimensions. Lane `min-width` set to `10rem / 160px`.

- [x] **Reorganize data folder structure** — Moved config and index into the `rojekti/` subfolder alongside cards. Exe looks for `rojekti/rojekti.config.yaml` relative to CWD then exe location.

- [x] **Card edit modal draggable bug** — Fixed by setting `:draggable="false"` on the PrimeVue Dialog.

- [x] **Fix drag-and-drop reordering and lane switching** — Debugged `onEnd` handler in `Lane.vue`. SortableJS DOM changes are now correctly reflected in Vue state and persisted to the Rust backend.

- [x] **Fix settings saving (epics and tags)** — Resolved `serde` camelCase/snake_case mismatches. `save_board_config` command now correctly receives and writes all config data.

- [x] **Status reordering in settings** — SortableJS drag-and-drop in `BoardSettingsModal` for status lanes. Backend updated to use structured `Status { id, name }` objects.

- [x] **Tags: assignable from card modal** — Replaced static tag list with a PrimeVue `MultiSelect`. Selected tags render as removable chips.

- [x] **Tags: colors and reordering in settings** — Full color pickers and drag-and-drop reordering for both Epics and Tags in settings modal. Tags stored as structured objects with `id`, `name`, `color`.

- [x] **Renaming status/epics/tags carries over to cards** — ID-based matching implemented. Renaming a display name no longer breaks the link to existing cards.

- [x] **Creating card then editing creates duplicate** — Fixed. The optimistic card push in `createCard` now checks for an existing ID before pushing to avoid duplicates on reload.

- [x] **Implement real-time refresh from CLI changes** — `notify` v6 `RecommendedWatcher` watches `rojekti/` recursively. Background thread debounces events (2s quiet window). GUI writes suppressed via `last_gui_write` timestamp (4s window). `rojekti.index.yaml` and temp files filtered out. Frontend listens for `board-changed` event and calls `loadBoard()`.

- [x] **Soft delete cards** — Deleting a card moves it to `rojekti/deleted/` instead of permanently removing it. Deleted cards are not shown in the GUI or CLI listings.

- [x] **Collapsed lane information ordering** — Card count now appears at the top and lane name at the bottom when collapsed, both correctly rotated.
