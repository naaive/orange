extern crate notify;

use crate::idx_store::IdxStore;
use crate::utils;
use crate::utils::subs;
use log::{error, info};
use notify::{raw_watcher, Op, RawEvent, RecursiveMode, Watcher};
#[cfg(target_os = "linux")]
use std::os::unix::fs::MetadataExt;
#[cfg(target_os = "macos")]
use std::os::unix::fs::MetadataExt;
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;
use std::path;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::sync::Arc;

pub struct FsWatcher {
  index_store: Arc<IdxStore>,
  path: String,
}

impl FsWatcher {
  pub fn new(index_store: Arc<IdxStore>, path: String) -> FsWatcher {
    FsWatcher { index_store, path }
  }

  pub fn start(&mut self) {
    let (tx, rx) = channel();
    let mut watcher = raw_watcher(tx).unwrap();
    let wt_res = watcher.watch(self.path.as_str(), RecursiveMode::Recursive);
    if wt_res.is_err() {
      error!("{:?}", wt_res.err());
      error!("watch {} err ", self.path);
      return;
    }
    info!("fs watcher started");

    loop {
      match rx.recv() {
        Ok(RawEvent {
          path: Some(path),
          op: Ok(op),
          cookie: _,
        }) => {
          let path_str = path.to_str().unwrap();
          if path_str.contains("orangecachedata") {
            continue;
          }
          if Op::REMOVE & op == Op::REMOVE {
            self.index_store._del(path_str.to_string())
          };
          let result = path.metadata();
          match result {
            Ok(meta) => {
              let abs_path = path.to_str().unwrap().to_string();

              let name = Self::get_filename(&path);

              #[cfg(windows)]
              let _size = meta.file_size();
              #[cfg(unix)]
              let _size = meta.size();

              let is_dir = meta.is_dir();
              if is_dir {
                if let Some(path_str) = path.to_str() {
                  self.save_subs(path_str);
                }
              }
              if let Some(p) = path.parent() {
                if let Some(parent_str) = p.to_str() {
                  self.save_subs(parent_str);
                }
              }

              self.index_store.add(&name, &abs_path)
            }
            Err(_) => {}
          }
        }
        Ok(event) => error!("broken event: {:?}", event),
        Err(e) => error!("watch error: {:?}", e),
      }
    }
  }

  fn save_subs(&mut self, parent_str: &str) {
    let subs = subs(parent_str);
    for sub in subs {
      let sub_path = path::Path::new(sub.as_str());
      let name = sub_path
        .file_name()
        .map(|x| x.to_str().unwrap())
        .unwrap_or_default()
        .to_string();

      if let Ok(_m) = sub_path.metadata() {
        self.index_store.add(&name, sub.clone().as_str());
      }
    }
  }

  fn get_filename(path: &PathBuf) -> String {
    let name = path
      .file_name()
      .map(|x| x.to_str().unwrap())
      .unwrap_or_default()
      .to_string();
    name
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use notify::watcher;
  use std::time::Duration;

  #[test]
  fn t1() {

    // let mut watcher = FsWatcher::new(
    //   ,
    //   "/Users/jeff/CLionProjects/orangemac/src-tauri/target".to_string(),
    // );
    // watcher.start();
  }
  #[test]
  fn t2() {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
      .watch(
        "/Users/jeff/CLionProjects/orangemac/src-tauri/target/hi",
        RecursiveMode::Recursive,
      )
      .unwrap();

    loop {
      match rx.recv() {
        Ok(event) => println!("{:?}", event),
        Err(e) => println!("watch error: {:?}", e),
      }
    }
  }
  #[test]
  fn t3() {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering raw events.
    // The notification back-end is selected based on the platform.
    let mut watcher = raw_watcher(tx).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch("/", RecursiveMode::Recursive).unwrap();

    loop {
      match rx.recv() {
        Ok(RawEvent {
          path: Some(path),
          op: Ok(op),
          cookie,
        }) => {
          let x = path.to_str().unwrap();
          if x.contains("orangecachedata") {
            continue;
          }
          println!("{}", x);
          // println!("{:?} {:?} ({:?})", op, path, cookie)
        }
        Ok(event) => println!("broken event: {:?}", event),
        Err(e) => println!("watch error: {:?}", e),
      }
    }
  }
}

#[test]
fn t4() {
  let conf_path = format!("{}{}", utils::data_dir(), "/orangecachedata/conf");
  let idx_path = format!("{}{}", utils::data_dir(), "/orangecachedata/idx");

  let idx_store = Arc::new(IdxStore::new(&idx_path));
  let mut watcher = FsWatcher::new(idx_store, "/".to_string());
  watcher.start();
}
