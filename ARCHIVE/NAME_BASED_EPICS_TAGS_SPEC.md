# Spec: Migrate Epics and Tags from ID-based to Name-based

## Overview

Currently epics and tags use opaque timestamp IDs (`epic-1773924443930`, `tag-1773869127764`) stored on card files. This makes raw card files unreadable. This migration replaces IDs with the human-readable name directly on the card.

**Dependency:** This spec must be implemented before `RENAME_WAL_SPEC.md`, which describes the rename safety mechanism that name-based storage requires.

---

## Design

- Cards store the epic/tag **name** directly: `epic: My Feature`, `tags: [bug, frontend]`
- Epics and tags in config lose their `id` field — `name` becomes the unique key
- Names must be unique within epics and within tags (enforced in the UI and on save)
- Renaming an epic/tag updates all card files (see `RENAME_WAL_SPEC.md` for the safe rename mechanism)
- Deleting a tag or epic from config means **"stop offering it as an option"** — not **"remove it from existing cards"**. Tags and epics on cards are historical facts and must be preserved.

---

## Files to Change

### 1. `src-tauri/src/models.rs`

Remove `id` from `Epic` and `Tag`:

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Epic {
    pub name: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
    pub color: String,
}
```

`CardMeta.epic` and `CardMeta.tags` are already `Option<String>` and `Vec<String>` — no change needed. They now store names instead of IDs.

### 2. `src-tauri/src/storage.rs` — one-time migration on read

Add a migration function that detects old-format configs (epics/tags with `id` fields) and card files (epic/tag values that look like IDs). Run this once on startup before the board is served to the frontend.

Since `serde_yaml` will fail to deserialize the old `Epic`/`Tag` structs (they now lack `id`), use a separate migration struct:

```rust
#[derive(Deserialize)]
struct LegacyEpic {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Deserialize)]
struct LegacyTag {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Deserialize)]
struct LegacyBoardConfig {
    // same as BoardConfig but with LegacyEpic/LegacyTag
    pub epics: Vec<LegacyEpic>,
    pub tags: Vec<LegacyTag>,
    // ... other fields unchanged
}
```

Migration function:

```rust
pub fn migrate_ids_to_names(dir: &Path) -> Result<(), String> {
    let config_path = dir.join("rojekti").join("rojekti.config.yaml");
    let content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Could not read config: {}", e))?;

    // Detect if migration is needed by checking for `id:` fields in epics/tags section
    // Use LegacyBoardConfig to parse old format
    let legacy: LegacyBoardConfig = match serde_yaml::from_str(&content) {
        Ok(c) => c,
        Err(_) => return Ok(()), // already migrated or unrecognizable — skip
    };

    // Build ID → name maps
    let epic_map: std::collections::HashMap<String, String> = legacy.epics.iter()
        .map(|e| (e.id.clone(), e.name.clone()))
        .collect();
    let tag_map: std::collections::HashMap<String, String> = legacy.tags.iter()
        .map(|t| (t.id.clone(), t.name.clone()))
        .collect();

    // Rewrite all card files
    let files = list_card_files(dir)?;
    for file in files {
        let card_content = fs::read_to_string(&file)
            .map_err(|e| format!("Could not read {}: {}", file.display(), e))?;
        let (mut meta, body) = parse_card_file(&card_content)?;

        let mut changed = false;

        if let Some(epic_id) = &meta.epic {
            if let Some(epic_name) = epic_map.get(epic_id) {
                meta.epic = Some(epic_name.clone());
                changed = true;
            }
        }

        let new_tags: Vec<String> = meta.tags.iter().map(|t| {
            tag_map.get(t).cloned().unwrap_or_else(|| t.clone())
        }).collect();
        if new_tags != meta.tags {
            meta.tags = new_tags;
            changed = true;
        }

        if changed {
            let card = Card { meta, body };
            write_card(dir, &card)?;
        }
    }

    // Rewrite config without IDs
    // (write_board_config will serialize the new Epic/Tag structs without id)
    // Parse the full config using a hybrid approach — keep all other fields,
    // just drop the ids from epics/tags
    // ...write new config here...

    Ok(())
}
```

Call `migrate_ids_to_names` at the top of `lib.rs` `run()` before starting the watcher or serving any commands, but only if the board directory exists.

### 3. `src-tauri/src/commands.rs`

**`save_board_config`**: After saving, check if any epic or tag was renamed compared to the previous config and trigger a bulk card file update (see `RENAME_WAL_SPEC.md`). Deletion requires no card file changes — orphaned names stay on cards intentionally.

```rust
pub fn save_board_config(config: BoardConfig, state: State<AppState>) -> Result<(), String> {
    let _lock = state.write_lock.lock()...;
    *state.last_gui_write.lock()...? = std::time::Instant::now();

    // Rename detection handled by RENAME_WAL_SPEC.md
    // Deletion: no card cleanup needed — orphaned tags/epics are preserved on cards

    storage::write_board_config(..., &config)?;
    index::rebuild_index(&state.project_dir)?;
    Ok(())
}
```

### 4. `src/types/index.ts`

```ts
export interface Epic {
  name: string
  color: string
}

export interface Tag {
  name: string
  color: string
}
```

### 5. `src/components/BoardSettingsModal.vue`

- `addEpic`: remove `id: \`epic-${Date.now()}\`` from the pushed object
- `addTag`: remove `id: \`tag-${Date.now()}\`` from the pushed object
- `:key="epic.id"` → `:key="epic.name"` (use name as key since there's no ID)
- `:key="tag.id"` → `:key="tag.name"`
- Add uniqueness validation in `handleSave`: if two epics or two tags share a name, show an error toast and abort save

### 6. `src/components/Card.vue`

- `getTag(id)` → `getTag(name)` — look up tag by name instead of ID (variable rename only, logic unchanged)
- `epic` computed: `config.value?.epics.find(e => e.id === props.card.epic)` → `config.value?.epics.find(e => e.name === props.card.epic)`

### 7. `src/components/CardModal.vue`

- `<Select optionValue="id">` for epics → `optionValue="name"`
- `<MultiSelect optionValue="id">` for tags → `optionValue="name"`
- `getTag(id)` → `getTag(name)`

**Orphaned tag/epic handling:** When the modal opens, split the card's tags into known (name exists in `config.tags`) and orphaned (name not found in config). The MultiSelect only binds to known tags. Orphaned tags are displayed separately as read-only chips with a neutral style — the user can remove them manually but they are never silently dropped on save.

```ts
const knownTags = computed(() =>
  card.value.tags.filter(name => config.value?.tags.some(t => t.name === name))
)
const orphanedTags = computed(() =>
  card.value.tags.filter(name => !config.value?.tags.some(t => t.name === name))
)
```

On save, merge back before calling `updateCard`:
```ts
card.value.tags = [...knownTags.value, ...orphanedTags.value]
```

Orphaned tags in the template — shown below the MultiSelect alongside the known selected tags, with a distinct neutral style and a manual remove button:
```vue
<span
  v-for="name in orphanedTags"
  :key="name"
  class="orphaned-tag"
>
  {{ name }} <i class="pi pi-times" @click="removeTag(name)" />
</span>
```

The same logic applies to the epic field: if `card.epic` is set but the name no longer exists in `config.epics`, display it as a read-only orphaned value rather than silently clearing it on save.

---

## What Not to Do

- Do not change `Status` — statuses keep their explicit `id` field since they are referenced by cards as IDs and the rename/delete logic for statuses is already implemented separately
- Do not add a uniqueness constraint at the Rust level for now — validate in the frontend only
- Do not run the migration if the board hasn't been initialized yet
- Do not remove orphaned tags or epics from card files when their config entry is deleted — this is intentional preservation of historical data
