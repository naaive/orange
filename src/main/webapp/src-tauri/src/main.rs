#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::process::Command;
use std::thread;

fn main() {
    thread::spawn(|| {
        Command::new("./lib/orange_core.exe")
            .output()
            .expect("failed to execute process");
    });

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
