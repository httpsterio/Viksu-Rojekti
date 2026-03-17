// Remove the windows_subsystem attribute to make this a console app by default
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use rojekti_lib::cli::{Cli, handle_cli};
use std::path::PathBuf;

#[cfg(windows)]
mod win_util {
    #[link(name = "user32")]
    extern "system" {
        pub fn ShowWindow(hwnd: isize, cmd_show: i32) -> i32;
    }
    #[link(name = "kernel32")]
    extern "system" {
        pub fn GetConsoleWindow() -> isize;
    }
}

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
    None
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        // CLI Mode: Handle command and exit before Tauri starts
        let project_dir = discover_project_dir().unwrap_or_else(|| {
            std::env::current_dir().unwrap_or_default()
        });
        handle_cli(command, project_dir);
        std::process::exit(0);
    }

    // GUI Mode: Hide console window on Windows
    #[cfg(windows)]
    unsafe {
        let hwnd = win_util::GetConsoleWindow();
        if hwnd != 0 {
            // SW_HIDE = 0
            win_util::ShowWindow(hwnd, 0);
        }
    }

    // Launch GUI
    rojekti_lib::run()
}
