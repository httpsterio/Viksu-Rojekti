# Feature Spec: Global Hotkey System

## Overview

Rojekti needs a global keyboard shortcut system that allows quick access to actions like creating cards, opening search, and navigating the board — without interfering with text editing within the app.

---

## Requirements

### 1. Guard: When Hotkeys Should Be Suppressed

Global hotkeys must be ignored when the user is actively editing text. This includes:

- Native inputs: `<input>`, `<textarea>`
- ContentEditable elements: any element with `contenteditable="true"` (used by some PrimeVue components)
- md-editor-v3: the editor renders its textarea/CodeMirror inside `.md-editor-input-wrapper` — check for `target.closest('.md-editor-input-wrapper')`
- PrimeVue selects and dropdowns when their input is active

The guard should be checked **first** on every keydown event before any handler is evaluated.

### 2. Guard: When the Modal is Open

The PrimeVue Dialog intercepts Escape natively to close itself. Global hotkeys (other than Escape) should still work when the modal is open unless the editor has focus (see above). Consider whether Escape should be a registerable hotkey at all — if so, it should only fire when no modal/dialog is open.

### 3. Key Notation

Use a normalized string format for key combinations:

- `mod` for Ctrl on Windows/Linux and Cmd on macOS (check `e.metaKey || e.ctrlKey`)
- `shift` for Shift
- Plain lowercase letter or key name for the base key (`e.key.toLowerCase()`)
- Parts joined with `+`, e.g. `mod+k`, `mod+shift+n`, `n`

### 4. File Structure

Three-file separation:

```
src/
├── composables/
│   └── useHotkeys.ts       # Mechanism only: guard logic, key normalization, event listener lifecycle
├── hotkeys.ts              # Registry only: the key→action map, imports actions and board state
└── App.vue                 # Wiring only: calls useHotkeys(hotkeys) — no hotkey logic here
```

**`useHotkeys.ts`** knows nothing about what the hotkeys do. It only handles:
- `isEditingText` guard
- key string normalization
- attaching/detaching the `window` keydown listener

**`hotkeys.ts`** is the only file you touch to add, remove, or change a shortcut. It imports whatever composables or actions it needs (e.g. `useBoard`) and exports the handler map. Example shape:

```ts
// src/hotkeys.ts
import { useBoard } from '@/composables/useBoard'

export function createHotkeyMap() {
  const { openCreateCard } = useBoard()

  return {
    'n': () => openCreateCard(),
    'mod+k': (e: KeyboardEvent) => { e.preventDefault(); /* open search */ },
  }
}
```

**`App.vue`** just wires them:

```ts
import { useHotkeys } from '@/composables/useHotkeys'
import { createHotkeyMap } from '@/hotkeys'

useHotkeys(createHotkeyMap())
```

### 5. Composable API

`useHotkeys(handlers)`:

- Accepts a `Record<string, (e: KeyboardEvent) => void>`
- Attaches a single `keydown` listener on `window` via `onMounted` / `onUnmounted`
- Handlers call `e.preventDefault()` themselves when needed — composable does not assume

### 6. Planned Hotkeys

| Key | Action | Notes |
|-----|--------|-------|
| `n` | Create new card | Should not fire if modal is open |
| `mod+k` | Open search/command palette | Future feature |
| `Escape` | Close modal | Already handled natively by PrimeVue Dialog — do not duplicate |

---

## Implementation Notes

- No external hotkey library needed — the guard logic and key normalization are simple enough to own
- The `isEditingText` guard function should be exported separately so it can be reused or tested independently
- If a handler is registered for a key that matches a browser default (e.g. `mod+s`), the handler is responsible for calling `e.preventDefault()`
- Consider debouncing or using `keydown` vs `keyup` — `keydown` is preferred for responsiveness and repeat-key suppression (hold key fires once via `e.repeat` check if needed)
