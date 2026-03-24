use std::path::PathBuf;
use tauri::Manager;
use crate::models::AppState;

pub mod models;
pub mod storage;
pub mod index;
pub mod commands;
pub mod cli;
pub mod watcher;

fn discover_project_dir() -> Option<PathBuf> {
    // 1. Check current working directory for a 'rojekti' folder
    if let Ok(cwd) = std::env::current_dir() {
        if cwd.join("rojekti").join("rojekti.config.yaml").exists() {
            return Some(cwd);
        }
        // 1.b Check parent directory (common in development)
        if let Some(parent) = cwd.parent() {
            if parent.join("rojekti").join("rojekti.config.yaml").exists() {
                return Some(parent.to_path_buf());
            }
        }
    }
    // 2. Check directory containing the executable for a 'rojekti' folder
    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            if exe_dir.join("rojekti").join("rojekti.config.yaml").exists() {
                return Some(exe_dir.to_path_buf());
            }
            // 2.b Check parent of executable directory (e.g., target/debug/..)
            if let Some(parent) = exe_dir.parent() {
                if parent.join("rojekti").join("rojekti.config.yaml").exists() {
                    return Some(parent.to_path_buf());
                }
            }
        }
    }
    None
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let project_dir = discover_project_dir().unwrap_or_else(|| {
        std::env::current_dir().unwrap_or_default()
    });

    let last_gui_write = std::sync::Arc::new(std::sync::Mutex::new(std::time::Instant::now()));

    tauri::Builder::default()
        .manage(AppState { 
            project_dir: project_dir.clone(),
            write_lock: std::sync::Mutex::new(()),
            last_gui_write: last_gui_write.clone(),
            watcher: std::sync::Mutex::new(None),
        })
        .setup(move |app| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
            }

            // Recover any interrupted rename operations from a previous session
            let config_path = project_dir.join("rojekti").join("rojekti.config.yaml");
            if config_path.exists() {
                if let Ok(mut config) = storage::read_board_config(&config_path) {
                    // One-time ID migration: assign UUIDs to any entry missing one
                    if storage::ensure_ids(&mut config) {
                        if let Err(e) = storage::write_board_config(&config_path, &config) {
                            eprintln!("[warn] Failed to save config after ID migration: {}", e);
                        }
                    }

                    let has_pending = config.epics.iter().any(|e| e.pending_rename.is_some())
                        || config.tags.iter().any(|t| t.pending_rename.is_some())
                        || config.statuses.iter().any(|s| s.pending_rename.is_some());
                    if has_pending {
                        if let Err(e) = storage::apply_pending_renames(&project_dir, &mut config, &config_path) {
                            eprintln!("[warn] Failed to recover pending rename (epics/tags): {}", e);
                        }
                        if let Err(e) = storage::apply_pending_status_renames(&project_dir, &mut config, &config_path) {
                            eprintln!("[warn] Failed to recover pending rename (statuses): {}", e);
                        }
                    }
                }
            }

            // Start the watcher immediately if the board already exists
            let rojekti_dir = project_dir.join("rojekti");
            if rojekti_dir.exists() {
                let state: tauri::State<AppState> = app.state();
                match watcher::start(project_dir.clone(), app.handle().clone(), last_gui_write.clone()) {
                    Ok(w) => {
                        *state.watcher.lock().unwrap() = Some(w);
                    }
                    Err(e) => eprintln!("Watcher failed to start: {}", e),
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_board_config,
            commands::save_board_config,
            commands::get_board_state,
            commands::save_board_state,
            commands::get_all_cards,
            commands::get_card,
            commands::create_card,
            commands::update_card,
            commands::delete_card,
            commands::move_card,
            commands::reorder_status,
            commands::rebuild_index,
            commands::init_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
