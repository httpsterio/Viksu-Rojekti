# Rojekti Backlog

## 🟢 Easy Wins
*High impact, low technical risk. Good for immediate polish.*

- [x] **Rename "Tickets" to "Cards"**
Standardize naming across the entire stack. This involves a bulk search-and-replace in Rust models, Vue components, TypeScript interfaces, and renaming the `tickets/` directory to `cards/`. (COMPLETED)

- [ ] **Fix Markdown Editor width mismatch**
The "Edit" tab currently appears wider than the "View" tab. This is likely due to default padding or container constraints in the `md-editor-v3` library. A surgical CSS fix in `CardModal.vue` will align them.

- [ ] **Default window size and responsiveness**
The app currently shows a horizontal scrollbar with five lanes. We need to adjust the CSS flex logic (likely `flex-basis` and `min-width`) and the default window dimensions in `tauri.conf.json` so lanes shrink gracefully to fit the viewport. Set lane minwidth to 10rem / 160px 

- [x] **Reorganize Data Folder Structure**
Move `board.yaml` and `index.yaml` into the same directory as the cards (the `rojekti/` folder). This keeps the project root clean and groups all "database" files together. (COMPLETED)

---

## 🔴 Serious Work
*Requires deeper logic changes, debugging, or new dependencies.*

- [x] **Fix Drag-and-Drop Reordering and Lane Switching**
Currently, dragging a card does not persist the change. This is likely a synchronization issue: SortableJS modifies the DOM, but the Vue reactive state and the Rust backend aren't receiving the correct new position or status. Requires debugging the `onEnd` handler in `Lane.vue`.

- [x] **Fix Settings Saving (including Epics and Tags)**
The UI for settings, epics, and tags is built, but changes aren't persisting. We need to verify the "plumbing": checking for `serde` naming mismatches (camelCase vs snake_case) and ensuring the `save_board_config` Tauri command is correctly receiving and writing the data.

- [ ] **Implement Real-Time Refresh from CLI Changes**
If an agent edits a card via the CLI, the GUI doesn't know. We need to implement a file watcher using the `notify` crate in Rust. When a file changes, the backend should emit a Tauri event to the frontend to trigger a `loadBoard()` refresh. Debounce-time or time between scans should be at least 5 seconds. Or refresh happens about 5 seconds since the last file change.

- [ ] **Lane Reordering in Settings**
Adding the ability to swap lane order in the `BoardSettingsModal`. This requires a SortableJS implementation inside the modal and updating the `lanes: Vec<String>` in the config file.

- [ ] **Internationalization (Translations)**
The app currently has hardcoded English strings. To support translations, we need to extract all text into a dedicated system (like `vue-i18n`) and replace hardcoded text with keys.

- [x] **Creating Card and moving bug**
Creating a card, then editing its' content (status etc.) creates a duplicate of the card that persists until a reload of the app.

---

## 🔍 Needs Clarification
*Requires user input or verification before proceeding.*

- [x] **Status of CLI "Unregister Class" Error**
  We recently switched the app to a **Console Subsystem** build to fix CLI output issues. We need to confirm if the `ERROR:ui\gfx\win\window_impl.cc:124` still appears.

- [ ] **Tags are listed and saved from settings but not assignable**
  A card cannot be assigned a tag via the GUI even if the tags appear in the edit modal. Creating tags in settings works, tags are surfaced to the modal but they can't be selected at all.

  Add a multiselect where the options come from the tags created in settings. allow selecting multiple. When clicking an option in the multiselect, add it as a chip under the search bar. The chip is removable and has a little cross button to delete/unselecting that that. 

- [ ] **Tags colors and reordering in settings missing**
  Tag order can't be reordered and tags have no color settings like epics have. We need them. Full color picker, same component as with Epics.

- [ ] **Renaming status/epics/tags should carry new name over to files using those**
  If we rename f.ex. "to-do" status to "TODO", all cards assigned to "to-do" should be assigned to "TODO" when saving the rename.
  Either refer to status/epic/tag with an ID and use string text only in UI to display OR update actual status/epic/tag name for each card when renaming them.
  Rewriting the tags instead of storing an ID is maybe better, more human readable and when using the CLI API we don't have to fetch and match ID to strings.


- [ ] **Collapsed lane information reordering**
  When a lane is collapsed, the lane name is rotated correctly. The amount of cards number in that lane is not rotated. The number should also follow the lane name, meaning the lane name is on the "bottom" and number on the "top" 

  So if a lane is normally [ Lane name (number of tickets) ], when it's rotated the lane name should be on the bottom going up towards the top and closer to the top end of the lane is the number of tickets.

- [ ] **Allow attachments in cards**
  Cards should allow attaching files to them, either as an online/local file link OR by copying the attachment to a rojekti/attachments folder. Need to ensure unique filenames. When attaching it should ask if we want to link or copy the file to attachments.

  File attachments (non-image) should add a link to the file locally or online that can be clicked to navigate to the item.
  Image attachments should display a thumbnail of the image that can be clicked to view with the default image viewer.

- [ ] **Deleting files**
  Deleting a card should move the card in its' current status (notes, status, epic, tags etc.) into rojekti/deleted instead of deleting the card from the filesystem. Deleted cards will not be shown in the GUI nor when using the CLI (unless we add a list --deleted or something)