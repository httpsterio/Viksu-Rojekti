use tauri::State;
use chrono::Local;
use std::fs;
use crate::models::{AppState, BoardConfig, Card, CardMeta, Index, Status, AllCardsResult, Priority};
use crate::{storage, index};

#[tauri::command]
pub fn get_board_config(state: State<AppState>) -> Result<BoardConfig, String> {
    storage::read_board_config(&state.project_dir.join("rojekti").join("rojekti.config.yaml"))
}

#[tauri::command]
pub fn save_board_config(mut config: BoardConfig, state: State<AppState>) -> Result<(), String> {
    let _lock = state.write_lock.lock().map_err(|e| format!("Lock error: {}", e))?;
    *state.last_gui_write.lock().map_err(|e| format!("Lock error: {}", e))? = std::time::Instant::now();
    let config_path = state.project_dir.join("rojekti").join("rojekti.config.yaml");

    let old_config = storage::read_board_config(&config_path)?;

    // Collect renames before mutating config (position-stable: same index = same entry)
    let epic_renames: Vec<(usize, String, String)> = config.epics.iter().enumerate()
        .filter_map(|(i, new)| {
            old_config.epics.get(i)
                .filter(|old| old.name != new.name)
                .map(|old| (i, old.name.clone(), new.name.clone()))
        })
        .collect();
    let tag_renames: Vec<(usize, String, String)> = config.tags.iter().enumerate()
        .filter_map(|(i, new)| {
            old_config.tags.get(i)
                .filter(|old| old.name != new.name)
                .map(|old| (i, old.name.clone(), new.name.clone()))
        })
        .collect();

    for (index, old_name, new_name) in epic_renames {
        storage::rename_epic_or_tag(&state.project_dir, &mut config, true, index, &old_name, &new_name, &config_path)?;
    }
    for (index, old_name, new_name) in tag_renames {
        storage::rename_epic_or_tag(&state.project_dir, &mut config, false, index, &old_name, &new_name, &config_path)?;
    }

    storage::write_board_config(&config_path, &config)
}

#[tauri::command]
pub fn get_all_cards(state: State<AppState>) -> Result<AllCardsResult, String> {
    let (cards, errors) = storage::read_all_cards(&state.project_dir)?;
    Ok(AllCardsResult { cards, errors })
}

#[tauri::command]
pub fn get_card(id: String, state: State<AppState>) -> Result<Card, String> {
    storage::read_card(&state.project_dir.join("rojekti").join("cards").join(format!("{}.md", id)))
}

#[tauri::command]
pub fn create_card(
    title: String,
    status: Option<String>,
    epic: Option<String>,
    tags: Vec<String>,
    priority: u8,
    body: String,
    state: State<AppState>,
) -> Result<Card, String> {
    let _lock = state.write_lock.lock().map_err(|e| format!("Lock error: {}", e))?;
    *state.last_gui_write.lock().map_err(|e| format!("Lock error: {}", e))? = std::time::Instant::now();
    
    let mut config = storage::read_board_config(&state.project_dir.join("rojekti").join("rojekti.config.yaml"))?;
    
    let cards_dir = state.project_dir.join("rojekti").join("cards");
    let deleted_dir = state.project_dir.join("rojekti").join("deleted");
    let id: String;
    loop {
        let candidate = format!("{}-{:03}", config.prefix, config.next_id);
        config.next_id += 1;
        if !cards_dir.join(format!("{}.md", candidate)).exists() 
            && !deleted_dir.join(format!("{}.md", candidate)).exists() {
            id = candidate;
            break;
        }
    }
    storage::write_board_config(&state.project_dir.join("rojekti").join("rojekti.config.yaml"), &config)?;
    
    let status = status.unwrap_or_else(|| config.statuses.first().map(|s| s.id.clone()).unwrap_or_default());
    
    let (cards, _) = storage::read_all_cards(&state.project_dir)?;
    let max_pos = cards.iter()
        .filter(|c| c.meta.status == status)
        .map(|c| c.meta.position)
        .fold(0.0, f64::max);
    
    let card = Card {
        meta: CardMeta {
            id,
            title,
            status,
            epic,
            tags,
            priority,
            position: max_pos + 1.0,
            created: Local::now().format("%Y-%m-%d").to_string(),
        },
        body,
    };
    
    storage::write_card(&state.project_dir, &card)?;
    index::rebuild_index(&state.project_dir)?;
    
    Ok(card)
}

#[tauri::command]
pub fn update_card(card: Card, state: State<AppState>) -> Result<Card, String> {
    let _lock = state.write_lock.lock().map_err(|e| format!("Lock error: {}", e))?;
    *state.last_gui_write.lock().map_err(|e| format!("Lock error: {}", e))? = std::time::Instant::now();
    storage::write_card(&state.project_dir, &card)?;
    index::rebuild_index(&state.project_dir)?;
    Ok(card)
}

#[tauri::command]
pub fn delete_card(id: String, state: State<AppState>) -> Result<(), String> {
    let _lock = state.write_lock.lock().map_err(|e| format!("Lock error: {}", e))?;
    *state.last_gui_write.lock().map_err(|e| format!("Lock error: {}", e))? = std::time::Instant::now();
    storage::delete_card_file(&state.project_dir, &id)?;
    index::rebuild_index(&state.project_dir)?;
    Ok(())
}

#[tauri::command]
pub fn move_card(
    id: String,
    new_status: String,
    new_position: f64,
    state: State<AppState>,
) -> Result<Card, String> {
    let _lock = state.write_lock.lock().map_err(|e| format!("Lock error: {}", e))?;
    *state.last_gui_write.lock().map_err(|e| format!("Lock error: {}", e))? = std::time::Instant::now();
    let mut card = storage::read_card(&state.project_dir.join("rojekti").join("cards").join(format!("{}.md", id)))?;
    card.meta.status = new_status;
    card.meta.position = new_position;
    
    storage::write_card(&state.project_dir, &card)?;
    index::rebuild_index(&state.project_dir)?;
    
    Ok(card)
}

#[tauri::command]
pub fn reorder_status(
    status: String,
    card_ids: Vec<String>,
    state: State<AppState>,
) -> Result<(), String> {
    let _lock = state.write_lock.lock().map_err(|e| format!("Lock error: {}", e))?;
    *state.last_gui_write.lock().map_err(|e| format!("Lock error: {}", e))? = std::time::Instant::now();
    for (i, id) in card_ids.iter().enumerate() {
        let mut card = storage::read_card(&state.project_dir.join("rojekti").join("cards").join(format!("{}.md", id)))?;
        card.meta.position = (i + 1) as f64;
        card.meta.status = status.clone();
        storage::write_card(&state.project_dir, &card)?;
    }
    index::rebuild_index(&state.project_dir)?;
    Ok(())
}

#[tauri::command]
pub fn rebuild_index(state: State<AppState>) -> Result<Index, String> {
    index::rebuild_index(&state.project_dir)
}

#[tauri::command]
pub fn init_project(
    name: String,
    prefix: String,
    app_handle: tauri::AppHandle,
    state: State<AppState>,
) -> Result<(), String> {
    validate_name_and_prefix(&name, &prefix)?;

    let _lock = state.write_lock.lock().map_err(|e| format!("Lock error: {}", e))?;
    *state.last_gui_write.lock().map_err(|e| format!("Lock error: {}", e))? = std::time::Instant::now();
    
    let config = BoardConfig {
        name,
        prefix,
        next_id: 1,
        statuses: vec![
            Status { id: "backlog".into(), name: "Backlog".into() },
            Status { id: "todo".into(), name: "Todo".into() },
            Status { id: "in-progress".into(), name: "In Progress".into() },
            Status { id: "review".into(), name: "Review".into() },
            Status { id: "done".into(), name: "Done".into() },
        ],
        epics: Vec::new(),
        tags: Vec::new(),
        priorities: vec![
            Priority { name: "Critical".into(),    color: "#e53e3e".into() },
            Priority { name: "Severe".into(),      color: "#ed8936".into() },
            Priority { name: "Substantial".into(), color: "#ecc94b".into() },
            Priority { name: "Moderate".into(),    color: "#68d391".into() },
            Priority { name: "Low".into(),         color: "#a0aec0".into() },
        ],
    };
    
    let rojekti_dir = state.project_dir.join("rojekti");
    if !rojekti_dir.exists() {
        fs::create_dir_all(&rojekti_dir)
            .map_err(|e| format!("Could not create rojekti directory: {}", e))?;
    }
    
    storage::write_board_config(&rojekti_dir.join("rojekti.config.yaml"), &config)?;
    fs::create_dir_all(rojekti_dir.join("cards"))
        .map_err(|e| format!("Could not create cards directory: {}", e))?;
    index::rebuild_index(&state.project_dir)?;
    
    // Start the watcher now that the directory exists
    let mut watcher_guard = state.watcher.lock().map_err(|e| format!("Lock error: {}", e))?;
    if watcher_guard.is_none() {
        match crate::watcher::start(state.project_dir.clone(), app_handle, state.last_gui_write.clone()) {
            Ok(w) => { *watcher_guard = Some(w); }
            Err(e) => eprintln!("Watcher failed to start after init: {}", e),
        }
    }
    
    Ok(())
}

fn validate_name_and_prefix(name: &str, prefix: &str) -> Result<(), String> {
    // Windows reserved device names (case-insensitive)
    const RESERVED: &[&str] = &[
        "CON", "PRN", "AUX", "NUL",
        "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
        "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];

    // Illegal filename characters on Windows
    const ILLEGAL_CHARS: &[char] = &['\\', '/', ':', '*', '?', '"', '<', '>', '|'];

    let prefix_upper = prefix.to_uppercase();
    if RESERVED.contains(&prefix_upper.as_str()) {
        return Err(format!(
            "'{}' is a reserved Windows device name and cannot be used as a card prefix.",
            prefix
        ));
    }

    for ch in ILLEGAL_CHARS {
        if name.contains(*ch) {
            return Err(format!(
                "Board name contains an illegal character: '{}'",
                ch
            ));
        }
        if prefix.contains(*ch) {
            return Err(format!(
                "Card prefix contains an illegal character: '{}'",
                ch
            ));
        }
    }

    if prefix.trim().is_empty() {
        return Err("Card prefix cannot be empty.".into());
    }

    if name.trim().is_empty() {
        return Err("Board name cannot be empty.".into());
    }

    Ok(())
}
