use tauri::State;
use chrono::Local;
use std::fs;
use crate::models::{AppState, BoardConfig, Ticket, TicketMeta, Index};
use crate::{storage, index};

#[tauri::command]
pub fn get_board_config(state: State<AppState>) -> Result<BoardConfig, String> {
    storage::read_board_config(&state.project_dir.join("board.yaml"))
}

#[tauri::command]
pub fn save_board_config(config: BoardConfig, state: State<AppState>) -> Result<(), String> {
    storage::write_board_config(&state.project_dir.join("board.yaml"), &config)
}

#[tauri::command]
pub fn get_all_tickets(state: State<AppState>) -> Result<Vec<Ticket>, String> {
    storage::read_all_tickets(&state.project_dir)
}

#[tauri::command]
pub fn get_ticket(id: String, state: State<AppState>) -> Result<Ticket, String> {
    storage::read_ticket(&state.project_dir.join("tickets").join(format!("{}.md", id)))
}

#[tauri::command]
pub fn create_ticket(
    title: String,
    status: Option<String>,
    epic: Option<String>,
    tags: Vec<String>,
    priority: String,
    body: String,
    state: State<AppState>,
) -> Result<Ticket, String> {
    let mut config = storage::read_board_config(&state.project_dir.join("board.yaml"))?;
    
    let id = format!("{}-{:03}", config.prefix, config.next_id);
    config.next_id += 1;
    storage::write_board_config(&state.project_dir.join("board.yaml"), &config)?;
    
    let status = status.unwrap_or_else(|| config.lanes.first().cloned().unwrap_or_default());
    
    // Calculate position: highest position in target lane + 1.0
    let tickets = storage::read_all_tickets(&state.project_dir)?;
    let max_pos = tickets.iter()
        .filter(|t| t.meta.status == status)
        .map(|t| t.meta.position)
        .fold(0.0, f64::max);
    
    let ticket = Ticket {
        meta: TicketMeta {
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
    
    storage::write_ticket(&state.project_dir, &ticket)?;
    index::rebuild_index(&state.project_dir)?;
    
    Ok(ticket)
}

#[tauri::command]
pub fn update_ticket(ticket: Ticket, state: State<AppState>) -> Result<Ticket, String> {
    storage::write_ticket(&state.project_dir, &ticket)?;
    index::rebuild_index(&state.project_dir)?;
    Ok(ticket)
}

#[tauri::command]
pub fn delete_ticket(id: String, state: State<AppState>) -> Result<(), String> {
    storage::delete_ticket_file(&state.project_dir, &id)?;
    index::rebuild_index(&state.project_dir)?;
    Ok(())
}

#[tauri::command]
pub fn move_ticket(
    id: String,
    new_status: String,
    new_position: f64,
    state: State<AppState>,
) -> Result<Ticket, String> {
    let mut ticket = storage::read_ticket(&state.project_dir.join("tickets").join(format!("{}.md", id)))?;
    ticket.meta.status = new_status;
    ticket.meta.position = new_position;
    
    storage::write_ticket(&state.project_dir, &ticket)?;
    index::rebuild_index(&state.project_dir)?;
    
    Ok(ticket)
}

#[tauri::command]
pub fn reorder_lane(
    status: String,
    ticket_ids: Vec<String>,
    state: State<AppState>,
) -> Result<(), String> {
    for (i, id) in ticket_ids.iter().enumerate() {
        let mut ticket = storage::read_ticket(&state.project_dir.join("tickets").join(format!("{}.md", id)))?;
        ticket.meta.position = (i + 1) as f64;
        ticket.meta.status = status.clone(); // Ensure it matches the requested lane
        storage::write_ticket(&state.project_dir, &ticket)?;
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
    let config = BoardConfig {
        name,
        prefix,
        next_id: 1,
        lanes: vec!["backlog".into(), "todo".into(), "in-progress".into(), "review".into(), "done".into()],
        epics: Vec::new(),
        tags: Vec::new(),
        priorities: vec!["low".into(), "medium".into(), "high".into(), "critical".into()],
    };
    
    if !state.project_dir.exists() {
        fs::create_dir_all(&state.project_dir)
            .map_err(|e| format!("Could not create project directory: {}", e))?;
    }
    
    storage::write_board_config(&state.project_dir.join("board.yaml"), &config)?;
    fs::create_dir_all(state.project_dir.join("tickets"))
        .map_err(|e| format!("Could not create tickets directory: {}", e))?;
    index::rebuild_index(&state.project_dir)?;
    
    Ok(())
}
