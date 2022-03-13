#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crate::file_view::FileView;
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

use std::sync::Arc;

use crate::idx_store::IdxStore;
use crate::kv_store::KvStore;

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
      let arc = IDX_STORE.clone().unwrap();
      if kw.eq("") {
        kw = "*".to_string();
      }
      let vec = arc.search(kw, 100);
      Ok(CustomResponse {
        message: "".to_string(),
        file_views: vec,
      })
    },
    // suggest
    2 => unsafe {
      let arc = IDX_STORE.clone().unwrap();
      let vec = arc.search(kw, 20);
      Ok(CustomResponse {
        message: "".to_string(),
        file_views: vec,
      })
    },
    _ => Ok(CustomResponse {
      message: "".to_string(),
      file_views: vec![],
    }),
  };
}

fn main() {
  std::thread::spawn(|| {
    indexing::run();
  });

  show();
}

fn show() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
