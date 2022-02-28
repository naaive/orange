#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[macro_use]
mod kv_store;
mod file_index;
mod file_kv;
mod file_view;
mod fs_walker;
mod fs_watcher;
mod index_store_v2;
mod united_store;
mod utils;

use crate::fs_walker::FsWalker;
use crate::fs_watcher::FsWatcher;
use crate::united_store::UnitedStore;
use index_store_v2::IndexStore;

use std::sync::{Arc, RwLock};
use std::time::Duration;
// Definition in main.rs

use crate::file_view::FileView;
use crate::kv_store::KvStore;
use tauri::{Manager, Window, Wry};

static mut FRONT_USTORE: Option<UnitedStore> = None;
static mut WINDOW: Option<Window<Wry>> = None;

struct Database {
  x: usize,
}

#[derive(serde::Serialize)]
struct CustomResponse {
  message: String,
  other_val: usize,
  file_views: Vec<FileView>,
}

#[tauri::command]
async fn my_custom_command(
  _window: Window<Wry>,
  number: usize,
  mut kw: String,
  database: tauri::State<'_, Database>,
) -> Result<CustomResponse, String> {
  return match number {
    // open file
    1 => {
      utils::open_file_path(kw.as_str());
      Ok(CustomResponse {
        message: "".to_string(),
        other_val: database.x,
        file_views: vec![],
      })
    }

    // search
    0 => unsafe {
      let arc = FRONT_USTORE.clone().unwrap();
      if kw.eq("") {
        kw = "*".to_string();
      }
      let vec = arc.search(kw.as_str(), 50);
      Ok(CustomResponse {
        message: "".to_string(),
        other_val: database.x,
        file_views: vec,
      })
    },
    // suggest
    2 => unsafe {
      let arc = FRONT_USTORE.clone().unwrap();
      let vec = arc.search(kw.as_str(), 20);
      Ok(CustomResponse {
        message: "".to_string(),
        other_val: database.x,
        file_views: vec,
      })
    },
    _ => Ok(CustomResponse {
      message: "".to_string(),
      other_val: database.x,
      file_views: vec![],
    }),
  };
}

const STORE_PATH: &'static str = "orangecachedata";

fn main() {
  // std::thread::spawn(|| {
  //   std::thread::sleep(Duration::from_secs(2));
  //   unsafe {
  //     utils::msg(WINDOW.clone().unwrap());
  //   }
  //
  // });
  let store = UnitedStore::new();

  let ustore = Arc::new(RwLock::new(store.clone()));
  let clone_store = ustore.clone();

  unsafe {
    FRONT_USTORE = Some(store.clone());
  }
  let kv_store = KvStore::new("./orangecachedata/conf");

  if cfg!(target_os = "windows") {
    #[cfg(windows)]
    unsafe {
      let drives = utils::get_win32_ready_drives();

      let drivs_clone = drives.clone();
      std::thread::spawn(move || loop {
        let home = utils::home_dir();
        let sub_home = utils::home_sub_dir();

        let mut walker = FsWalker::new(
          clone_store.clone(),
          sub_home.clone(),
          vec![STORE_PATH.to_string(), "$RECYCLE.BIN".to_string()],
          kv_store.clone(),
        );
        std::thread::spawn(move || {
          walker.start();
        });

        let sub_root = utils::sub_root();
        for sub in sub_root {
          let uclone1 = ustore.clone();

          let mut walker = FsWalker::new(
            clone_store.clone(),
            sub.clone(),
            vec![
              home.clone(),
              STORE_PATH.to_string(),
              "$RECYCLE.BIN".to_string(),
            ],
            kv_store.clone(),
          );
          std::thread::spawn(move || {
            walker.start();
          });
        }

        std::thread::sleep(Duration::from_secs(3600 * 24 * 1))
      });

      for driv in drives {
        let uclone1 = ustore.clone();
        let driv_clone1 = driv.clone();

        std::thread::spawn(move || {
          let mut watcher = FsWatcher::new(uclone1, driv_clone1);
          watcher.start();
        });
      }
    }
  } else {
    let sub_root = utils::sub_root();
    for sub in sub_root {
      let uclone1 = ustore.clone();
      std::thread::spawn(move || {
        let mut watcher = FsWatcher::new(uclone1, sub.clone());
        watcher.start();
      });
    }

    let home = utils::home_dir();
    let sub_home = utils::home_sub_dir();

    std::thread::spawn(move || loop {
      // Path::new(home)
      let mut walker = FsWalker::new(
        clone_store.clone(),
        sub_home.clone(),
        vec![STORE_PATH.to_string()],
        kv_store.clone(),
      );
      walker.start();

      let mut walker = FsWalker::new(
        clone_store.clone(),
        vec!["/".to_string()],
        vec![home.clone(), STORE_PATH.to_string()],
        kv_store.clone(),
      );
      walker.start();

      std::thread::sleep(Duration::from_secs(3600 * 1))
    });
  }

  start_tauri_app();
}

fn start_tauri_app() {
  let database = Database { x: 123 };
  tauri::Builder::default()
    .setup(|x1| {
      let option = x1.get_window("main");
      unsafe {
        WINDOW = option;
      }
      Ok(())
    })
    .manage(database)
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
