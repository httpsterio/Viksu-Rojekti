use std::path::PathBuf;
use tauri::Manager;
use tauri_plugin_cli::CliExt;
use crate::models::AppState;

pub mod models;
pub mod storage;
pub mod index;
pub mod commands;
pub mod cli;

fn discover_project_dir() -> Option<PathBuf> {
    // 1. Check current working directory
    if let Ok(cwd) = std::env::current_dir() {
        if cwd.join("board.yaml").exists() {
            return Some(cwd);
        }
        // 1.b Check parent directory (common in development)
        if let Some(parent) = cwd.parent() {
            if parent.join("board.yaml").exists() {
                return Some(parent.to_path_buf());
            }
        }
    }
    // 2. Check directory containing the executable
    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            if exe_dir.join("board.yaml").exists() {
                return Some(exe_dir.to_path_buf());
            }
            // 2.b Check parent of executable directory (e.g., target/debug/..)
            if let Some(parent) = exe_dir.parent() {
                if parent.join("board.yaml").exists() {
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
        .plugin(tauri_plugin_cli::init())
        .manage(AppState { project_dir: project_dir.clone() })
        .setup(move |app| {
            match app.cli().matches() {
                Ok(matches) => {
                    if matches.subcommand.is_some() {
                        cli::handle_cli(matches, project_dir);
                        
                        // Exit the process immediately for CLI commands
                        // This prevents the GUI from initializing further
                        #[cfg(windows)]
                        unsafe {
                            #[link(name = "kernel32")]
                            extern "system" {
                                fn FreeConsole() -> i32;
                            }
                            FreeConsole();
                        }
                        std::process::exit(0);
                    } else {
                        // No subcommand: show the main window
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                        }
                    }
                }
                Err(_) => {
                    // No CLI args: show the main window
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                    }
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_board_config,
            commands::save_board_config,
            commands::get_all_tickets,
            commands::get_ticket,
            commands::create_ticket,
            commands::update_ticket,
            commands::delete_ticket,
            commands::move_ticket,
            commands::reorder_lane,
            commands::rebuild_index,
            commands::init_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
