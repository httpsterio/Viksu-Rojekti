// Keep as console app to ensure PowerShell/CMD waits for output in CLI mode.
// We will programmatically hide the console in GUI mode.

use clap::Parser;
use rojekti_lib::cli::{Cli, handle_cli};
use std::path::PathBuf;

fn discover_project_dir() -> Option<PathBuf> {
    if let Ok(cwd) = std::env::current_dir() {
        if cwd.join("board.yaml").exists() {
            return Some(cwd);
        }
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
        let project_dir = discover_project_dir().unwrap_or_else(|| {
            std::env::current_dir().unwrap_or_default()
        });
        handle_cli(command, project_dir);
        std::process::exit(0);
    }

    // GUI Mode: Detach from console if we were launched via double-click (spawned our own console)
    #[cfg(windows)]
    unsafe {
        use windows::Win32::System::Console::{GetConsoleProcessList, FreeConsole};
        let mut process_list = [0u32; 2];
        let count = GetConsoleProcessList(&mut process_list);
        if count == 1 {
            // Only detach if we are the only process using this console.
            // This prevents hiding the terminal if run from PowerShell/CMD.
            let _ = FreeConsole();
        }
    }

    rojekti_lib::run()
}
