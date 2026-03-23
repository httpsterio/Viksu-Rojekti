# Manual UI Tests

Run these before any release or after changes to the frontend. Each scenario is independent — no shared state assumed unless a precondition is listed.

---

## Cards

### Create a card

**Preconditions:** Board is open with at least one status column.

1. Click the + button on any column header
2. Type a title
3. Press Enter or confirm

**Expected:** Card appears at the bottom of that column. It has an assigned ID (`prefix-XXX`). Opening it shows the correct status, today's date, and empty body.

---

### Edit a card

**Preconditions:** At least one card exists.

1. Click a card to open it
2. Change the title, body, priority, epic, or tags
3. Save

**Expected:** Changes are reflected immediately in the card modal and on the board. Reopening the card shows the updated values. The card file on disk contains the new data.

---

### Delete a card

**Preconditions:** At least one card exists.

1. Open a card
2. Delete it

**Expected:** Card disappears from the board. The file is moved to `rojekti/deleted/` — not permanently removed. The ID is not reused if a new card is created afterward.

---

### Move a card between columns

**Preconditions:** At least two status columns exist with at least one card.

1. Drag a card from one column and drop it into another

**Expected:** Card appears in the target column immediately. Its status in the file on disk matches the new column. Position within the column is reasonable (not 0, not the same as another card).

---

### Reorder cards within a column

**Preconditions:** At least two cards in the same column.

1. Drag one card above or below another within the same column

**Expected:** The order on screen matches the drag result. After reloading the board, the order is preserved. Positions on disk reflect the new order.

---

## Search and filters

### Search by title

1. Type a partial word from a card title into the search field

**Expected:** Only cards whose titles contain the search string are shown, across all columns. Cards that don't match are hidden but columns remain visible.

---

### Search by card ID

1. Type a card ID (e.g. `roj-042`) into the search field

**Expected:** Only that card is shown. Partial ID search works (e.g. `042` or `roj-04`).

---

### Filter by epic

1. Click an epic in the filter bar

**Expected:** Only cards assigned to that epic are shown. Cards with no epic or a different epic are hidden.

---

### Filter by tag

1. Click a tag in the filter bar

**Expected:** Only cards that include that tag are shown.

---

### Combine filters

1. Apply a search term and an epic filter simultaneously

**Expected:** Only cards that satisfy both conditions are shown. Clearing one filter restores the broader result set.

---

### Clear filters

1. Apply any filter
2. Clear it (click again, or clear the search field)

**Expected:** All cards return to view. No ghost filters remain.

---

## Settings

### Rename an epic

**Preconditions:** At least one epic exists and is assigned to at least one card.

1. Open settings
2. Rename the epic
3. Save

**Expected:** The epic name updates everywhere — in the filter bar, on the cards that had it assigned, and in the card files on disk. No card retains the old epic name.

---

### Rename a tag

**Preconditions:** At least one tag exists and is used on at least one card.

1. Open settings
2. Rename the tag
3. Save

**Expected:** Same as epic rename — all cards that used the tag now show the new name. Old name is gone from disk.

---

### Add a new status

1. Open settings
2. Add a new status with a name
3. Save

**Expected:** A new column appears on the board. It is empty. Existing cards are unaffected.

---

### Reorder statuses

1. Open settings
2. Change the order of statuses
3. Save

**Expected:** Columns on the board appear in the new order. Cards stay in their correct column.

---

## Views

### Switch to epic view

1. Switch from board view to epic view

**Expected:** Cards are grouped by epic. Cards with no epic appear in an "unassigned" group. Active filters still apply.

---

### Collapse a column

1. Click the collapse toggle on a status column header

**Expected:** The column collapses and cards are hidden. Other columns are unaffected. Clicking again restores the column.

---

## Resilience

### Load a board with a malformed card file

**Preconditions:** Manually corrupt one card file (e.g. remove a `---` delimiter or break the YAML).

1. Open or reload the board

**Expected:** The board loads successfully. A warning toast appears identifying the problematic file. All other cards load normally. The app does not crash or show a blank board.

---

### Reload after external file change

**Preconditions:** Board is open.

1. Manually edit a card file on disk (change the title in the frontmatter)
2. Wait a moment for the file watcher to pick it up

**Expected:** The board reflects the change without requiring a manual reload.
