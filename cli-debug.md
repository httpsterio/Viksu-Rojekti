# CLI and Output Synchronization Debug Report

## 🔍 The Findings: Why the CLI is "Buggy"

Claude is **correct**. The root cause of the interleaving output and the "hanging" process is that the CLI logic is trapped inside Tauri's lifecycle hooks.

### 1. The Setup Hook Race Condition
Currently, the CLI dispatch happens inside the `.setup()` closure in `lib.rs`. 
*   **Current State:** When you run `.\rojekti.exe list`, the application starts, initializes the Tauri engine, sets up the internal webview hooks, and *then* checks for CLI matches in the setup hook.
*   **The Problem:** By the time `.setup()` runs, Tauri has already started parts of its event loop. Calling `std::process::exit(0)` here is like pulling the emergency brake on a moving train. It leaves "ghost" hooks in the Windows windowing system (hence your `Failed to unregister class` errors) and often exits before the OS-level pipe has finished flushing the stdout buffer.

### 2. Subsystem and Shell Synchronization
Even though we switched to a **Console Subsystem**, because Tauri is being initialized, the shell thinks the process has "branched" or is preparing to open a window. 
*   The PowerShell prompt returns because it sees the Tauri engine start and assumes the "GUI" part is taking over.
*   The output then trickles in *after* the prompt is already rendered, leading to the overwriting behavior.

---

## 💻 Current vs. Proposed Code

### Current (The "Trapped" CLI)
In `lib.rs`, we are using the `tauri-plugin-cli`. This is the "Tauri way," but it's bad for pure CLI performance:

```rust
// lib.rs
tauri::Builder::default()
    .plugin(tauri_plugin_cli::init())
    .setup(|app| {
        let matches = app.cli().matches()?; // <--- Too late! Tauri has already started.
        if matches.subcommand.is_some() {
            cli::handle_cli(matches, dir);
            std::process::exit(0); // <--- Pulling the emergency brake.
        }
        Ok(())
    })
```

### Proposed (The "Pure" CLI)
To fix this, we should move the CLI parsing to `main.rs` before the `tauri::Builder` is even created. 

```rust
// main.rs
fn main() {
    let args: Vec<String> = std::env::args().collect();

    // 1. Check for CLI args BEFORE Tauri starts
    if args.len() > 1 && !args[1].starts_with("--") {
        // Use a simple manual parser or a crate like 'clap'
        // to handle commands like 'list', 'show', 'create'.
        cli::handle_pure_rust(&args); 
        
        // 2. Explicitly flush stdout
        std::io::stdout().flush().unwrap();
        
        // 3. Exit before Tauri engine is even touched
        std::process::exit(0);
    }

    // 4. Only if no CLI args, launch the heavy GUI engine
    rojekti_lib::run();
}
```

---

## 🚀 Why this fixes it:
1.  **Instant Output:** No Tauri initialization means the `list` command runs in ~10ms instead of ~500ms.
2.  **Shell Blocking:** Since the process never tries to spin up a WebView2 engine, Windows treats it as a standard console command. PowerShell will wait for the process to exit before showing the next prompt.
3.  **No Ghost Errors:** No `Chrome_WidgetWin` errors because the webview library was never loaded into memory.

## 📋 Summary for Claude:
"The current implementation uses `tauri-plugin-cli` inside the `.setup()` hook. This causes the Tauri engine to initialize fully before the CLI command is handled. On Windows, this leads to a race condition where the shell prompt returns before the output is finished, and `exit(0)` causes cleanup errors in the WebView2 hooks. Moving CLI dispatch to `main()` before `tauri::Builder` is initialized is the only way to ensure clean shell synchronization."
