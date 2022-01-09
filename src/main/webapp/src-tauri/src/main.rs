#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::process::Command;
use std::{process, thread};
use tauri::{Manager, SystemTray, SystemTrayEvent};
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
use tauri::api::path::BaseDirectory::Runtime;

fn main() {

    thread::spawn(|| {
        Command::new("orange_core.exe")
            .output()
            .expect("failed to execute process");
    });

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);


    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {

                match id.as_str() {
                    "quit" => {
                        process::exit(0x0100);
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
