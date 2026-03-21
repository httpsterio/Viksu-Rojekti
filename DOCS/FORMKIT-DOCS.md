# FormKit Drag-and-Drop Library Documentation

This document summarizes the public API surface of the FormKit drag-and-drop library based on the source code.

## 1. useDragAndDrop Signature

The `useDragAndDrop` composable is the primary entry point for Vue applications.

```typescript
export function useDragAndDrop<T>(
  initialValues: T[],
  options: Partial<VueParentConfig<T>> = {}
): [
  Ref<HTMLElement | undefined>,
  Ref<T[]>,
  (config: Partial<VueParentConfig<T>>) => void
]
```

### Parameters
- `initialValues`: An array of items to be managed by the drag-and-drop instance.
- `options`: A configuration object (see [Config Options](#2-all-config-options)).

### Return Values
- `parent`: A Vue template ref to be attached to the container element.
- `values`: A reactive ref containing the current state of the list.
- `updateConfig`: A function to dynamically update the configuration.

---

## 2. All Config Options

Configuration options are defined in the `ParentConfig<T>` interface.

| Option | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `disabled` | `boolean` | `false` | Disables drag-and-drop for the entire parent. |
| `sortable` | `boolean` | `true` | Whether items can be reordered within the parent. |
| `group` | `string` | `undefined` | Items can be transferred between parents with the same group name. |
| `dragHandle` | `string` | `undefined` | A CSS selector to restrict dragging to a specific element within the node. |
| `nativeDrag` | `boolean` | `true` | Whether to use the native HTML5 Drag and Drop API. If `false`, uses a synthetic implementation. |
| `draggable` | `(child: HTMLElement) => boolean` | `undefined` | A function to determine if a specific DOM element is draggable. |
| `draggingClass` | `string` | `undefined` | Class added to the node while it is being dragged. |
| `dragPlaceholderClass` | `string` | `undefined` | Class added to the original node while its clone is being dragged. |
| `dropZoneClass` | `string` | `undefined` | Class added to a node when another node is dragged over it. |
| `threshold` | `{ horizontal: number; vertical: number }` | `{ horizontal: 0, vertical: 0 }` | Sensitivity for triggering a sort (0 to 1). |
| `plugins` | `Array<DNDPlugin>` | `[]` | An array of plugins (e.g., animations, insert). |
| `accepts` | `Function` | `undefined` | Advanced logic to determine if a parent accepts a dragged node. |

---

## 3. Drag Handles

Drag handles are implemented using the `dragHandle` option, which accepts a CSS selector.

```typescript
// Example config
{
  dragHandle: ".drag-handle"
}
```

The library validates the handle by checking if the pointer-down event originated from an element matching the selector:

```typescript
export function validateDragHandle<T>({ x, y, node, config }: { ... }): boolean {
  if (!config.dragHandle) return true;
  const dragHandles = node.el.querySelectorAll(config.dragHandle);
  const elFromPoint = config.root.elementFromPoint(x, y);
  for (const handle of Array.from(dragHandles))
    if (elFromPoint === handle || handle.contains(elFromPoint)) return true;
  return false;
}
```

---

## 4. Animations

Animations are enabled via the `animations` plugin.

### Configuration
```typescript
import { animations } from "@formkit/drag-and-drop";

useDragAndDrop(items, {
  plugins: [
    animations({
      duration: 150, // Animation duration in ms
      xScale: 0.5,   // Horizontal scale offset
      yScale: 0.5    // Vertical scale offset
    })
  ]
});
```

### How it works
The plugin taps into the `setupNodeRemap` hook and uses the Web Animations API to slide elements into their new positions:

```typescript
node.animate(animation, {
  duration: duration,
  easing: "ease-in-out",
});
```

---

## 5. Restricting Drag Axis & List

### Within a Single List
To restrict dragging within one list only, simply **omit the `group` option**. Without a shared group or an `accepts` function, items cannot be moved between parents.

### Y-Axis Only Restriction
The core library does not have a native "lock-to-axis" configuration for the visual drag image. However, sorting logic sensitivity can be adjusted via the `threshold` option:

```typescript
{
  threshold: {
    horizontal: 1, // Requires crossing entire width to trigger horizontal sort
    vertical: 0    // Triggers vertical sort immediately
  }
}
```

---

## 6. Transfer Between Lists

To move items between lists, parents must share a `group` name or use the `accepts` callback.

```typescript
// List A
const [parentA, valuesA] = useDragAndDrop(itemsA, { group: "todo-list" });

// List B
const [parentB, valuesB] = useDragAndDrop(itemsB, { group: "todo-list" });
```

The `performTransfer` function handles the logic of removing from source and inserting into target:

```typescript
export function performTransfer<T>({ currentParent, targetParent, ... }) {
  // ... logic to update both parent values ...
  setParentValues(currentParent.el, currentParent.data, currentParentValues);
  setParentValues(targetParent.el, targetParent.data, targetParentValues);
}
```

---

## 7. Events and Callbacks

The library provides several lifecycle hooks in the `ParentConfig`.

| Hook | Signature | Description |
| :--- | :--- | :--- |
| `onDragstart` | `(data: DragstartEventData<T>) => void` | Fired when a drag operation begins. |
| `onDragend` | `(data: DragendEventData<T>) => void` | Fired when a drag operation completes. |
| `onSort` | `(data: SortEventData<T>) => void` | Fired when items are reordered within the same parent. |
| `onTransfer` | `(data: TransferEventData<T>) => void` | Fired when an item is moved to a different parent. |

### Example
```typescript
onSort: (data) => {
  console.log("Moved item from", data.previousPosition, "to", data.position);
}
```

---

## 8. Drop Zone / Insertion Indicators

The library uses classes and an optional `insert` plugin for visual feedback.

### CSS Classes
- `dropZoneClass`: Applied to the node currently being dragged over.
- `dragPlaceholderClass`: Applied to the original node while dragging.

### Insert Plugin (Insertion Line)
The `insert` plugin creates a dedicated element to indicate the insertion point between items.

```typescript
import { insert } from "@formkit/drag-and-drop";

plugins: [
  insert({
    insertPoint: (parent) => {
      const el = document.createElement("div");
      el.className = "my-insertion-line";
      return el;
    }
  })
]
```

The plugin calculates the `range` for each node and positions the `insertPoint` element absolutely:

```typescript
Object.assign(insertState.insertPoint.el.style, {
  top: `${topPosition}px`,
  left: `${position.x[0]}px`,
  // ...
});
```
