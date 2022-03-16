#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crate::file_view::FileView;
use tauri::{SystemTray, SystemTrayEvent};
use tauri::{Window, Wry};

mod file_view;
mod fs_watcher;
mod idx_store;
mod indexing;
mod kv_store;
mod pinyin_tokenizer;
#[cfg(windows)]
mod usn_journal_watcher;
mod utils;
mod walk_exec;
mod watch_exec;

use crate::idx_store::IdxStore;
use crate::kv_store::KvStore;
use std::sync::Arc;
use std::time::Duration;
use tauri::{CustomMenuItem, SystemTrayMenu};

static mut IDX_STORE: Option<Arc<IdxStore>> = None;
static mut CONF_STORE: Option<Arc<KvStore>> = None;

#[derive(serde::Serialize)]
struct CustomResponse {
  message: String,
  file_views: Vec<FileView>,
}

#[tauri::command]
async fn my_custom_command(
  _window: Window<Wry>,
  number: usize,
  mut kw: String,
) -> Result<CustomResponse, String> {
  return match number {
    // open file
    1 => {
      utils::open_file_path(kw.as_str());
      Ok(CustomResponse {
        message: "".to_string(),
        file_views: vec![],
      })
    }

    // search
    0 => unsafe {
      if IDX_STORE.is_none() {
        Ok(CustomResponse {
          message: "".to_string(),
          file_views: vec![],
        })
      } else {
        let arc = IDX_STORE.clone().unwrap();
        if kw.eq("") {
          kw = "*".to_string();
        }
        let vec = arc.search(kw, 100);
        Ok(CustomResponse {
          message: "".to_string(),
          file_views: vec,
        })
      }
    },
    // suggest
    2 => unsafe {
      let arc = IDX_STORE.clone().unwrap();
      let vec = arc.suggest(kw, 20);
      Ok(CustomResponse {
        message: "".to_string(),
        file_views: vec,
      })
    },

    // open file in terminal
    3 => {
      utils::open_file_path_in_terminal(kw.as_str());
      Ok(CustomResponse {
        message: "".to_string(),
        file_views: vec![],
      })
    }

    _ => Ok(CustomResponse {
      message: "".to_string(),
      file_views: vec![],
    }),
  };
}

fn main() {
  utils::init_log();

  std::thread::spawn(|| {
    std::thread::sleep(Duration::from_secs(1));
    indexing::run();
  });

  show();
}

fn show() {
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let tray_menu = SystemTrayMenu::new().add_item(quit);
  let tray = SystemTray::new().with_menu(tray_menu);

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .system_tray(tray)
    .on_system_tray_event(|_app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
        "quit" => {
          std::process::exit(0);
        }
        _ => {}
      },
      _ => {}
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
