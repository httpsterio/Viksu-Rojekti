use notify::{RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};

const DEBOUNCE_SECS: u64 = 2;
const SUPPRESS_SECS: u64 = 4;

pub fn start(
    project_dir: PathBuf,
    app_handle: AppHandle,
    last_gui_write: Arc<Mutex<Instant>>,
) -> Result<RecommendedWatcher, String> {
    let watch_dir = project_dir.join("rojekti");

    let (tx, rx) = std::sync::mpsc::channel::<Result<Event, notify::Error>>();

    let mut watcher = RecommendedWatcher::new(
        move |res| { let _ = tx.send(res); },
        notify::Config::default(),
    ).map_err(|e| format!("Failed to create watcher: {}", e))?;

    watcher
        .watch(&watch_dir, RecursiveMode::Recursive)
        .map_err(|e| format!("Failed to watch directory: {}", e))?;

    std::thread::spawn(move || {
        loop {
            // Block until first event arrives
            match rx.recv() {
                Err(_) => break, // sender dropped, app is shutting down
                Ok(event) => {
                    if should_ignore(&event) {
                        continue;
                    }
                    // Drain subsequent events for up to DEBOUNCE_SECS with resets
                    loop {
                        match rx.recv_timeout(Duration::from_secs(DEBOUNCE_SECS)) {
                            Ok(next_event) => {
                                if !should_ignore(&next_event) {
                                    // Another real change arrived — reset the debounce window
                                    continue;
                                }
                            }
                            Err(_) => break, // timeout: no events for 5 seconds — fire
                        }
                    }

                    // Check if the GUI was the writer
                    let suppress = {
                        let t = last_gui_write.lock().unwrap();
                        t.elapsed() < Duration::from_secs(SUPPRESS_SECS)
                    };

                    if !suppress {
                        let _ = app_handle.emit("board-changed", ());
                    }
                }
            }
        }
    });

    Ok(watcher)
}

fn should_ignore(event: &Result<Event, notify::Error>) -> bool {
    let event = match event {
        Ok(e) => e,
        Err(_) => return true,
    };

    // Only care about data modifications (not access times, metadata, etc.)
    match event.kind {
        EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_) => {}
        _ => return true,
    }

    // Only react to paths we explicitly care about
    !event.paths.iter().any(|p| is_watched(p))
}

fn is_watched(path: &std::path::Path) -> bool {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

    if name == "rojekti.config.yaml" {
        return true;
    }

    if name.ends_with(".md") {
        let parent = path.parent().and_then(|p| p.file_name()).and_then(|n| n.to_str()).unwrap_or("");
        return parent == "cards" || parent == "deleted";
    }

    false
}
