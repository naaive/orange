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
mod index_store;
mod pinyin_tokenizer;
mod united_store;

#[cfg(windows)]
mod usn_journal_watcher;

mod utils;

#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

use std::{fs, panic, thread};

use crate::fs_walker::FsWalker;
use crate::fs_watcher::FsWatcher;
use crate::united_store::UnitedStore;
use index_store::IndexStore;

use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, RwLock};
use std::time::Duration;
// Definition in main.rs
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
use tauri::{SystemTray, SystemTrayEvent};

use crate::file_view::FileView;
use crate::kv_store::KvStore;
#[cfg(windows)]
use crate::usn_journal_watcher::Watcher;
#[cfg(windows)]
use crate::utils::build_volume_path;
use crate::utils::parse_ts;
use tauri::{Manager, Window, Wry};

static mut FRONT_USTORE: Option<UnitedStore> = None;
static mut CONF_KV_STORE: Option<KvStore> = None;
static mut WINDOW: Option<Window<Wry>> = None;
static TOTAL_NANOS: u32 = 3_000_000;

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
      let vec = arc.search(kw.as_str(), 35);
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
const RECYCLE_PATH: &'static str = "$RECYCLE.BIN";
const VERSION: &'static str = "0.0.4";

fn main() {
  let mut kv_store = KvStore::new("./orangecachedata/conf");

  housekeeping(&mut kv_store);

  let store = UnitedStore::new();
  let ustore = Arc::new(RwLock::new(store.clone()));
  let clone_store = ustore.clone();

  unsafe {
    FRONT_USTORE = Some(store.clone());
    CONF_KV_STORE = Some(kv_store.clone());
  }

  if cfg!(target_os = "windows") {
    #[cfg(windows)]
    unsafe {
      let success = maybe_usn_watch();

      // travel
      let drives = utils::get_win32_ready_drives();
      // std::thread::spawn(move || loop {
      //   // home
      //   let home = utils::home_dir();
      //   let sub_home = utils::home_sub_dir();
      //   for sub in sub_home {
      //     let mut walker = FsWalker::new(
      //       clone_store.clone(),
      //       vec![sub.clone()],
      //       vec![STORE_PATH.to_string(), RECYCLE_PATH.to_string()],
      //       kv_store.clone(),
      //       TOTAL_NANOS as u64,
      //     );
      //     std::thread::spawn(move || {
      //       walker.start();
      //     });
      //   }
      //
      //   //total
      //   let sub_root = utils::sub_root();
      //   for sub in sub_root {
      //     // let uclone1 = ustore.clone();
      //
      //     let mut walker = FsWalker::new(
      //       clone_store.clone(),
      //       vec![sub.clone()],
      //       vec![
      //         home.clone(),
      //         STORE_PATH.to_string(),
      //         RECYCLE_PATH.to_string(),
      //       ],
      //       kv_store.clone(),
      //       TOTAL_NANOS as u64,
      //     );
      //     std::thread::spawn(move || {
      //       walker.start();
      //     });
      //   }
      //
      //   std::thread::sleep(Duration::from_secs(3600 * 24 * 1))
      // });

      if success {
        println!("usn watch success");
      } else {
        //watch
        for driv in drives {
          let uclone1 = ustore.clone();
          let driv_clone1 = driv.clone();

          std::thread::spawn(move || {
            let mut watcher = FsWatcher::new(uclone1, driv_clone1);
            let result = panic::catch_unwind(move || {
              watcher.start();
            });
            if result.is_err() {
              warm("bad watcher,{}");
            }
          });
        }
      }
    }
  } else {
    #[cfg(unix)]
    {
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

      std::thread::spawn(move || {
        loop {
          // Path::new(home)
          let nanos = 3 * 1000000;
          let mut walker = FsWalker::new(
            clone_store.clone(),
            sub_home.clone(),
            vec![STORE_PATH.to_string()],
            kv_store.clone(),
            nanos,
          );
          walker.start();

          let mut walker = FsWalker::new(
            clone_store.clone(),
            vec!["/".to_string()],
            vec![home.clone(), STORE_PATH.to_string()],
            kv_store.clone(),
            nanos,
          );
          walker.start();

          std::thread::sleep(Duration::from_secs(3600 * 1))
        }
      });
    }
  }

  start_tauri_app();
}

fn housekeeping(kv_store: &mut KvStore) {
  let version_opt = kv_store.get_str("version".to_string());
  match version_opt {
    None => {
      std::fs::remove_dir_all("orangecachedata/index");
      std::fs::remove_dir_all("orangecachedata/kv");
      kv_store.put_str("version".to_string(), VERSION.to_string());
      println!("init version {}", VERSION);
    }
    Some(version) => {
      if !version.eq(VERSION) {
        std::fs::remove_dir_all("orangecachedata/index");
        std::fs::remove_dir_all("orangecachedata/kv");
        kv_store.put_str("version".to_string(), VERSION.to_string());
        println!("clean old version cachedata");
      }
    }
  }
}

#[cfg(windows)]
unsafe fn maybe_usn_watch() -> bool {
  let (tx, rx) = mpsc::channel();
  let nos = utils::get_win32_ready_drives_no();

  for no in nos {
    let volume_path = build_volume_path(no.as_str());
    let tx_clone = tx.clone();
    start_usn_watch(no, volume_path, tx_clone);
  }

  let success = rx.recv().unwrap();
  success
}

#[cfg(windows)]
unsafe fn start_usn_watch<'a>(no: String, volume_path: String, tx_clone: Sender<bool>) {
  println!("start_usn_watch {}", volume_path);

  thread::spawn(move || {
    let mut kv_store = CONF_KV_STORE.clone().unwrap();
    let key = format!("usn#next_usn#{}", volume_path.clone());
    let next_usn = kv_store
      .get_str(key.clone())
      .unwrap_or("0".to_string())
      .parse()
      .unwrap();

    let result = Watcher::new(volume_path.as_str(), None, Some(next_usn));
    if result.is_err() {
      tx_clone.send(false).unwrap();
      return;
    }

    let mut watcher = result.unwrap();
    tx_clone.send(true);
    let mut loaded = false;
    loop {
      let read_res = watcher.read();
      if read_res.is_err() {
         watcher = Watcher::new(volume_path.as_str(), None, Some(0)).unwrap();
        continue;
      }
      let records = read_res.unwrap();
      if records.is_empty() {
        if !loaded {
          loaded = true;
          println!("volume {} usn history loaded", volume_path);
        }
        thread::sleep(Duration::from_millis(500));
      } else {
        let usn_no = records.last().unwrap().usn.to_string();

        for record in records {
          let path = record.path.to_str().unwrap();
          let file_name = record.file_name;
          let abs_path = format!("{}:{}", no.as_str(), path);

          match fs::metadata(abs_path.clone()) {
            Ok(meta) => {
              if abs_path.contains(STORE_PATH) || abs_path.contains(RECYCLE_PATH) {
                continue;
              }
              FRONT_USTORE.clone().unwrap().save(FileView {
                abs_path: abs_path.clone(),
                name: file_name,
                created_at: parse_ts(meta.created().ok().unwrap()),
                mod_at: parse_ts(meta.modified().ok().unwrap()),
                size: meta.file_size(),
                is_dir: meta.is_dir(),
              })
            }
            Err(_) => FRONT_USTORE.clone().unwrap().del(abs_path.clone().as_str()),
          }
        }

        kv_store.put_str(key.clone(), usn_no);
      }
    }
  });
}

unsafe fn warm(content: &str) {
  utils::msg(WINDOW.clone().unwrap(), content);
}

fn start_tauri_app() {
  // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  // let hide = CustomMenuItem::new("reindex".to_string(), "Reindex");
  let tray_menu = SystemTrayMenu::new()
    .add_item(quit)
    .add_native_item(SystemTrayMenuItem::Separator);

  let tray = SystemTray::new().with_menu(tray_menu);

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
    .system_tray(tray)
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
        "quit" => {
          std::process::exit(0);
        }
        "reindex" => {
          std::process::exit(0);
        }
        _ => {}
      },
      SystemTrayEvent::LeftClick { .. } => {}
      SystemTrayEvent::RightClick { .. } => {}
      SystemTrayEvent::DoubleClick { .. } => {}
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
