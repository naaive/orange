use crate::file_view::FileView;
use crate::idx_store::IdxStore;
use crate::idx_store::IDX_STORE;
use crate::kv_store::KvStore;
use crate::walk_metrics::WalkMatrixView;
use crate::{indexing, utils, walk_exec};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tauri::{App, Manager};
use tauri::{CustomMenuItem, SystemTrayMenu};
use tauri::{SystemTray, SystemTrayEvent};
use tauri::{Window, Wry};

static mut WINDOW: Option<Window<Wry>> = None;

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

#[tauri::command]
fn walk_metrics() -> WalkMatrixView {
  unsafe { walk_exec::get_walk_matrix() }
}

#[tauri::command]
async fn suggest(kw: String) -> Vec<FileView> {
  unsafe { IDX_STORE.suggest(kw, 20) }
}

#[tauri::command]
async fn search(
  mut kw: String,
  is_dir_opt: Option<bool>,
  ext_opt: Option<String>,
) -> Vec<FileView> {
  unsafe {
    if kw.eq("") {
      kw = "*".to_string();
    }
    IDX_STORE.search_with_filter(kw, 100, is_dir_opt, ext_opt, None)
  }
}

#[tauri::command]
fn open_file_in_terminal(kw: String) {
  utils::open_file_path_in_terminal(kw.as_str());
}

#[tauri::command]
fn open_file_path(kw: String) {
  utils::open_file_path(kw.as_str());
}

fn init_window(x: &mut App<Wry>) {
  let option = x.get_window("main");
  unsafe {
    WINDOW = option;
  }
}

fn handle_tray_event(event: SystemTrayEvent) {
  match event {
    SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
      "quit" => {
        std::process::exit(0);
      }
      "reindex" => {
        unsafe {
          WINDOW.clone().unwrap().emit(
            "reindex",
            Payload {
              message: "".to_string(),
            },
          );
        }
        indexing::reindex();
      }
      _ => {}
    },
    _ => {}
  }
}

fn build_tray() -> SystemTray {
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let reindex = CustomMenuItem::new("reindex".to_string(), "Reindex");
  let tray_menu = SystemTrayMenu::new().add_item(reindex).add_item(quit);
  let tray = SystemTray::new().with_menu(tray_menu);
  tray
}

pub fn show() {
  tauri::Builder::default()
    .setup(|x| {
      init_window(x);
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      open_file_path,
      open_file_in_terminal,
      search,
      suggest,
      walk_metrics
    ])
    .system_tray(build_tray())
    .on_system_tray_event(|_, event| handle_tray_event(event))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
