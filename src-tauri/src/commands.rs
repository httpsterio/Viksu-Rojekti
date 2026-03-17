use tauri::State;
use chrono::Local;
use std::fs;
use crate::models::{AppState, BoardConfig, Card, CardMeta, Index};
use crate::{storage, index};

#[tauri::command]
pub fn get_board_config(state: State<AppState>) -> Result<BoardConfig, String> {
    storage::read_board_config(&state.project_dir.join("rojekti").join("rojekti.config.yaml"))
}

#[tauri::command]
pub fn save_board_config(config: BoardConfig, state: State<AppState>) -> Result<(), String> {
    let _lock = state.write_lock.lock().map_err(|e| format!("Lock error: {}", e))?;
    storage::write_board_config(&state.project_dir.join("rojekti").join("rojekti.config.yaml"), &config)
}

#[tauri::command]
pub fn get_all_cards(state: State<AppState>) -> Result<Vec<Card>, String> {
    storage::read_all_cards(&state.project_dir)
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
    priority: String,
    body: String,
    state: State<AppState>,
) -> Result<Card, String> {
    let _lock = state.write_lock.lock().map_err(|e| format!("Lock error: {}", e))?;
    
    let mut config = storage::read_board_config(&state.project_dir.join("rojekti").join("rojekti.config.yaml"))?;
    
    let id = format!("{}-{:03}", config.prefix, config.next_id);
    config.next_id += 1;
    storage::write_board_config(&state.project_dir.join("rojekti").join("rojekti.config.yaml"), &config)?;
    
    let status = status.unwrap_or_else(|| config.lanes.first().cloned().unwrap_or_default());
    
    let cards = storage::read_all_cards(&state.project_dir)?;
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
    storage::write_card(&state.project_dir, &card)?;
    index::rebuild_index(&state.project_dir)?;
    Ok(card)
}

#[tauri::command]
pub fn delete_card(id: String, state: State<AppState>) -> Result<(), String> {
    let _lock = state.write_lock.lock().map_err(|e| format!("Lock error: {}", e))?;
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
    let mut card = storage::read_card(&state.project_dir.join("rojekti").join("cards").join(format!("{}.md", id)))?;
    card.meta.status = new_status;
    card.meta.position = new_position;
    
    storage::write_card(&state.project_dir, &card)?;
    index::rebuild_index(&state.project_dir)?;
    
    Ok(card)
}

#[tauri::command]
pub fn reorder_lane(
    status: String,
    card_ids: Vec<String>,
    state: State<AppState>,
) -> Result<(), String> {
    let _lock = state.write_lock.lock().map_err(|e| format!("Lock error: {}", e))?;
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
pub fn init_project(name: String, prefix: String, state: State<AppState>) -> Result<(), String> {
    let _lock = state.write_lock.lock().map_err(|e| format!("Lock error: {}", e))?;
    let config = BoardConfig {
        name,
        prefix,
        next_id: 1,
        lanes: vec!["backlog".into(), "todo".into(), "in-progress".into(), "review".into(), "done".into()],
        epics: Vec::new(),
        tags: Vec::new(),
        priorities: vec!["low".into(), "medium".into(), "high".into(), "critical".into()],
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
    
    Ok(())
}
