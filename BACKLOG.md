# Rojekti Backlog

## 🟢 Easy Wins
*High impact, low technical risk. Good for immediate polish.*

**Rename "Tickets" to "Cards"**
Standardize naming across the entire stack. This involves a bulk search-and-replace in Rust models, Vue components, TypeScript interfaces, and renaming the `tickets/` directory to `cards/`. (COMPLETED)

**Fix Markdown Editor width mismatch**
The "Edit" tab currently appears wider than the "View" tab. This is likely due to default padding or container constraints in the `md-editor-v3` library. A surgical CSS fix in `CardModal.vue` will align them.

**Default window size and responsiveness**
The app currently shows a horizontal scrollbar with five lanes. We need to adjust the CSS flex logic (likely `flex-basis` and `min-width`) and the default window dimensions in `tauri.conf.json` so lanes shrink gracefully to fit the viewport.

**Reorganize Data Folder Structure**
Move `board.yaml` and `index.yaml` into the same directory as the cards (the `rojekti/` folder). This keeps the project root clean and groups all "database" files together. (COMPLETED)

---

## 🔴 Serious Work
*Requires deeper logic changes, debugging, or new dependencies.*

**Fix Drag-and-Drop Reordering and Lane Switching**
Currently, dragging a card does not persist the change. This is likely a synchronization issue: SortableJS modifies the DOM, but the Vue reactive state and the Rust backend aren't receiving the correct new position or status. Requires debugging the `onEnd` handler in `Lane.vue`.

**Fix Settings Saving (including Epics and Tags)**
The UI for settings, epics, and tags is built, but changes aren't persisting. We need to verify the "plumbing": checking for `serde` naming mismatches (camelCase vs snake_case) and ensuring the `save_board_config` Tauri command is correctly receiving and writing the data.

**Implement Real-Time Refresh from CLI Changes**
If an agent edits a card via the CLI, the GUI doesn't know. We need to implement a file watcher using the `notify` crate in Rust. When a file changes, the backend should emit a Tauri event to the frontend to trigger a `loadBoard()` refresh.

**Lane Reordering in Settings**
Adding the ability to swap lane order in the `BoardSettingsModal`. This requires a SortableJS implementation inside the modal and updating the `lanes: Vec<String>` in the config file.

**Internationalization (Translations)**
The app currently has hardcoded English strings. To support translations, we need to extract all text into a dedicated system (like `vue-i18n`) and replace hardcoded text with keys.

---

## 🔍 Needs Clarification
*Requires user input or verification before proceeding.*

**Status of CLI "Unregister Class" Error**
We recently switched the app to a **Console Subsystem** build to fix CLI output issues. We need to confirm if the `ERROR:ui\gfx\win\window_impl.cc:124` still appears.

**Empty Tags Section Detail**
In the edit view, the tags section is empty. Does this mean the *available* tags defined in the board config are missing, or that the *assigned* tags on that specific card aren't loading? Knowing this helps distinguish between a config-load bug and a card-parse bug.
