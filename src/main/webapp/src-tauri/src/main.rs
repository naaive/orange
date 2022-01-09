#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::{process, thread};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::process::{Command, Stdio};

use tauri::{SystemTray, SystemTrayEvent};
use tauri::{CustomMenuItem, SystemTrayMenu};

static MAX_LOG_SIZE: u64 = 30 * 1024 * 1024;
static LOG_NAME: &str = "log/orange.log";

fn main() {
    thread::spawn(|| {
        let file_out = Stdio::from(open_file());

        Command::new("lib/orange_core.exe")
            // .stderr(file_out)
            .stderr(file_out)
            .output()
            .expect("failed to execute process");
        println!("hi")
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
                        process::exit(0);
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


fn open_file() -> File {
    let mut file;
    if Path::new(LOG_NAME).exists() {
        file = OpenOptions::new()
            .create_new(false)
            .write(true)
            .append(true)
            .open(LOG_NAME)
            .unwrap();
    } else {
        file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .append(true)
            .open(LOG_NAME)
            .unwrap();
        let len = file.metadata().unwrap().len();
        if len > MAX_LOG_SIZE {
            file = OpenOptions::new()
                .create_new(true)
                .write(true)
                .append(false)
                .open(LOG_NAME)
                .unwrap();
        }
    };
    file
}
