// Remove the windows_subsystem attribute to make this a console app by default
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // If no CLI arguments, hide the console window and launch GUI
    if args.len() == 1 {
        #[cfg(windows)]
        unsafe {
            let hwnd = win_util::GetConsoleWindow();
            if hwnd != 0 {
                // SW_HIDE = 0
                win_util::ShowWindow(hwnd, 0);
            }
        }
    }

    rojekti_lib::run()
}
