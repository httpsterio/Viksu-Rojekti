use tauri::State;
use chrono::Local;
use std::fs;
use crate::models::{AppState, BoardConfig, BoardState, Card, CardMeta, Index, Status, AllCardsResult, Priority};
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

    let mut old_config = storage::read_board_config(&config_path)?;

    // 1. Assign IDs to any new entries added by the frontend
    storage::ensure_ids(&mut config);

    // 2. Identify and mark renames (ID-based matching)
    let (has_epic_tag_renames, has_status_renames) = storage::detect_renames(&mut old_config, &config);

    // 3. If renames detected, write WAL and propagate
    if has_epic_tag_renames {
        storage::write_board_config(&config_path, &old_config)?;
        storage::apply_pending_renames(&state.project_dir, &mut old_config, &config_path)?;
    }

    if has_status_renames {
        storage::write_board_config(&config_path, &old_config)?;
        storage::apply_pending_status_renames(&state.project_dir, &mut old_config, &config_path)?;
    }

    // 4. Finally write the full new config (preserving IDs, colors, and order)
    storage::write_board_config(&config_path, &config)
}

#[tauri::command]
pub fn get_board_state(state: State<AppState>) -> BoardState {
    storage::read_board_state(&state.project_dir.join("rojekti").join("rojekti.state.yaml"))
}

#[tauri::command]
pub fn save_board_state(board_state: BoardState, state: State<AppState>) -> Result<(), String> {
    storage::write_board_state(
        &state.project_dir.join("rojekti").join("rojekti.state.yaml"),
        &board_state,
    )
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
    
    let status = status.unwrap_or_else(|| config.statuses.first().map(|s| s.name.clone()).unwrap_or_default());
    
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
    
    let done_id = uuid::Uuid::new_v4().to_string();
    let mut config = BoardConfig {
        name,
        prefix,
        next_id: 1,
        statuses: vec![
            Status { id: uuid::Uuid::new_v4().to_string(), name: "Backlog".into(), pending_rename: None },
            Status { id: uuid::Uuid::new_v4().to_string(), name: "Todo".into(), pending_rename: None },
            Status { id: uuid::Uuid::new_v4().to_string(), name: "In Progress".into(), pending_rename: None },
            Status { id: uuid::Uuid::new_v4().to_string(), name: "Review".into(), pending_rename: None },
            Status { id: done_id.clone(), name: "Done".into(), pending_rename: None },
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
        done_statuses: vec![done_id],
        hidden_statuses: Vec::new(),
        hidden_statuses_enabled: false,
    };
    
    storage::ensure_ids(&mut config);
    
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
