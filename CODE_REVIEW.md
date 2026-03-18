# Rojekti: Comprehensive Code Review & Technical Audit

This document outlines the findings of a deep-dive review into the Rojekti codebase (Rust backend and Vue/TS frontend).

---

## 🦀 Rust Backend (src-tauri)

### 1. `storage.rs`: I/O Robustness
- **Issue**: `parse_card_file` uses `splitn(3, "---")`. If a card's Markdown body contains `---` (common for horizontal rules), it correctly preserves them, but if the closing `---` is missing, it fails ungracefully.
- **Improvement**: Add a specific check for the existence of exactly two `---` delimiters before parsing.
- **Issue**: `write_card` overwrites files without a backup or atomic write. If the app crashes during a write, the card file could be corrupted or truncated.
- **Improvement**: Implement "write-then-rename" (write to a `.tmp` file, then `fs::rename` to the target).
- **Issue**: `delete_card_file` moves files to `rojekti/deleted/`. If multiple cards with the same name are deleted over time, `fs::rename` will overwrite the previous deleted card.
- **Improvement**: Append a timestamp to the filename in the `deleted/` folder.

### 2. `commands.rs`: Performance & Logic
- **Issue**: `create_card` reads **all** cards from disk just to calculate the `max_pos`. As the board grows to hundreds of cards, this will become slow.
- **Improvement**: Since we already have a `rojekti.index.yaml`, we should query the index for the max position, or maintain a local cache in `AppState`.
- **Issue**: `reorder_status` performs a separate `storage::write_card` call for every card in the status. This triggers multiple file-watcher events (though debounced) and is inefficient I/O.
- **Improvement**: Batch the writes if possible, though `std::fs` is synchronous.
- **Issue**: `rebuild_index` is called after every single write operation. Since the GUI primarily uses `get_all_cards`, the index is only strictly needed for the CLI. 
- **Improvement**: Consider making index rebuilding lazy or backgrounded.

### 3. `watcher.rs`: Thread Safety & Filtering
- **Issue**: `should_ignore` uses `event.paths.iter().all(...)`. If a single event contains both an ignorable and a non-ignorable file, it might be ignored entirely depending on the OS batching logic.
- **Improvement**: Change to `any` logic or more granular path filtering.
- **Issue**: The `last_gui_write` suppression window (4s) is close to the debounce window (2s). There's a small race condition where a GUI write could trigger a reload if the debounce outlasts the suppression.
- **Improvement**: Increase the buffer gap.

---

## ⚡ Frontend (Vue 3 / TypeScript)

### 1. `useBoard.ts`: State Management & Sync
- **Issue**: `loadBoard` sets `isLoading.value = true`. This triggers a full-screen spinner. If the watcher triggers a reload, the user's screen flashes, which is jarring.
- **Improvement**: Implement "Background Refresh." Only show the spinner on initial load. For watcher-triggered updates, refresh silently in the background and update the reactive `cards` array.
- **Issue**: `move_card` (drag-and-drop) is asynchronous but the GUI doesn't "optimistically" update the local state. If the backend is slow, the card might "snap back" before moving.
- **Improvement**: Update the local `cards` array immediately, then call the backend. Roll back only on failure.
- **Issue**: `filteredCards` and `cardsByStatus` are computed properties. They re-run their logic (filtering/sorting) whenever *any* card changes. On boards with 500+ cards, this may cause micro-stutter during dragging.
- **Improvement**: Memoize these or use a more performant sorting algorithm.

### 2. `CardModal.vue`: Data Integrity
- **Issue**: The modal uses a `Partial<Card>` for local state. When saving, it spreads this onto the existing card. 
- **Risk**: If new fields are added to the Rust model but not the Vue ref, they could be lost during a save.
- **Improvement**: Always work with a full `Card` object or a strict `EditCard` interface.

---

## 🛡️ General Safety & Performance

### 1. Race Conditions
- **Scenario**: A user is dragging a card (GUI write) while an external script is editing the same card (CLI write). 
- **Current State**: The `write_lock` in Rust prevents file corruption, but the "Last writer wins" rule applies. 
- **Risk**: The user's drag could be overwritten by the CLI's edit, or vice-versa, with no "Conflict Detected" warning.

### 2. Memory Usage
- **Rust**: The `AppState` is small and static. No significant leaks found.
- **Vue**: Ensure `listen('board-changed', ...)` in `App.vue` is properly cleaned up (unsubscribed) if the component unmounts (though `App.vue` rarely does).

---

## 🎨 UI Components & UX

### 1. `TopBar.vue`: Component Logic
- **Bug**: `const filterPanel = ref()` is defined but never assigned to the `<Popover ref="filterPanel">` because the variable name matches the ref attribute but isn't explicitly linked in a way that works if the build minifies names or if using certain setups. 
- **Improvement**: Ensure the ref is correctly linked and initialized.
- **Issue**: Stale Filters. If an Epic or Tag is deleted in Settings, the `activeFilters` in the TopBar might still hold the ID of the deleted item, resulting in an empty board with no obvious reason why.
- **Improvement**: Add a watcher in `useBoard.ts` to clear filters if the corresponding items are removed from the config.

### 2. `Card.vue`: Performance
- **Issue**: `getTag` and `getTagStyle` are called in a `v-for`. For a board with many cards, this leads to hundreds of redundant lookups into the `config.tags` array.
- **Improvement**: Pass the resolved tag objects to the Card component or use a computed Map for O(1) lookups.

### 3. `App.vue`: Resource Management
- **Issue**: `listen('board-changed', ...)` returns an unlisten function. Currently, this is ignored. While `App.vue` is the root, if it ever re-renders or if this pattern is copied to other components, it will leak listeners.
- **Improvement**: Store the unlisten function and call it in `onUnmounted`.

---

## 🔒 Security & Configuration

### 1. `tauri.conf.json`: CSP
- **Issue**: `"csp": null`. This disables Content Security Policy protection.
- **Improvement**: Define a strict CSP that only allows `default-src 'self'` and specific required sources (like `style-src 'unsafe-inline'` for PrimeVue).

### 2. `capabilities/default.json`: Permissions
- **Issue**: Only `core:default` is listed. 
- **Improvement**: Explicitly list permissions like `event:allow-listen` and `event:allow-emit` to follow the principle of least privilege.

---

## 🏗️ Architectural & Lifecycle Audit (New)

### 1. Stress Test: The "5000 Card" Scenario
- **Event**: A script creates 5000 cards in the `cards/` directory.
- **Expected Failure**: `useBoard.ts`'s `cardsByStatus` and `filteredCards` computed properties will re-calculate on every small change. Since they use `.filter()` and `.sort()` on the entire array, the GUI will likely lock up for several seconds during any drag-and-drop operation.
- **Recommendation**: Use a more efficient data structure (like an object/map indexed by Status ID) or implement virtual scrolling for the board.

### 2. Event: Invalid User-Edit of Card Files
- **Event**: A user manually edits a `.md` file and introduces a syntax error in the YAML frontmatter (e.g., `priority: [low`).
- **Expected Failure**: The Rust backend `read_card` will return an `Err`. `get_all_cards` will fail completely, meaning the **entire board disappears** because one card is broken.
- **Recommendation**: In `read_all_cards`, log the error for individual broken cards but continue loading the rest. Provide a visual "Broken Card" indicator in the UI.

### 3. Lifecycle: Closing App During Save
- **Event**: User clicks "Save" on a large card and immediately closes the window.
- **Expected Failure**: Tauri's backend process is killed. If the write was in progress, the file is corrupted.
- **Recommendation**: Use Tauri's `on_window_event` to intercept the close signal and check if `write_lock` is active or if a "isSaving" flag is true.

### 4. Event: Init with OS-Reserved Names
- **Event**: User initializes a board with the name `CON`, `PRN`, or `AUX` (on Windows).
- **Expected Failure**: Filesystem creation will fail or behave unpredictably because these are reserved Windows device names.
- **Recommendation**: Add a sanitization/validation layer to `init_project` to block reserved names and illegal path characters.

### 5. Multi-Instance Conflicts
- **Event**: Two instances of Rojekti are open. Instance A renames "Status 1" to "Doing". Instance B renames "Status 1" to "Work".
- **Expected Failure**: The `rojekti.config.yaml` will simply be overwritten by whoever clicked save last. Cards might end up pointing to a non-existent Status ID.
- **Recommendation**: Implement a "Config Version" or "E-Tag" system. Before saving, check if the file on disk has changed since it was last read.

---

## 📋 Recommended Priority Fixes (Finalized)
1. **Graceful Error Handling**: Don't let one broken YAML file kill the whole board load.
2. **Background Refresh**: (Urgent) Remove the jarring full-screen loading flash.
3. **Atomic Writes**: (Security) Use a `.tmp` and `rename` pattern to prevent data loss.
4. **Optimistic UI**: Fix the "snap-back" delay in drag-and-drop.
5. **Sanitization**: Validate Board Name and Card Prefix at creation time.

