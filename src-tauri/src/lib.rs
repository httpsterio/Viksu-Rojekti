use std::path::PathBuf;
use tauri::Manager;
use crate::models::AppState;

pub mod models;
pub mod storage;
pub mod index;
pub mod commands;
pub mod cli;

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

    tauri::Builder::default()
        .manage(AppState { 
            project_dir: project_dir.clone(),
            write_lock: std::sync::Mutex::new(()),
        })
        .setup(move |app| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_board_config,
            commands::save_board_config,
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
