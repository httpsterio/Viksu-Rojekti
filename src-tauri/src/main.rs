// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(windows)]
#[link(name = "kernel32")]
extern "system" {
    fn AttachConsole(dwProcessId: u32) -> i32;
}

fn main() {
    #[cfg(windows)]
    unsafe {
        // ATTACH_PARENT_PROCESS = -1 (or u32::MAX)
        AttachConsole(u32::MAX);
    }

    rojekti_lib::run()
}
