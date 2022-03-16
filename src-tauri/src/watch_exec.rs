use crate::idx_store::IdxStore;
#[cfg(windows)]
use crate::utils::get_win32_ready_drives;

use crate::fs_watcher::FsWatcher;
use crate::utils;
use std::sync::Arc;

pub fn run(idx_store: Arc<IdxStore>) {
  #[cfg(windows)]
  win_run(idx_store);

  if cfg!(target_os = "linux") {
    #[cfg(unix)]
    linux_run(idx_store);
  }
  if cfg!(target_os = "macos") {
    #[cfg(macos)]
    macos_run(idx_store);
  }
}

#[cfg(macos)]
fn macos_run(idx_store: Arc<IdxStore>) {
  let mut watcher = FsWatcher::new(idx_store.clone(), "/");
  std::thread::spawn(move || {
    watcher.start();
  });
}

#[cfg(unix)]
fn linux_run(idx_store: Arc<IdxStore>) {
  let sub_root = utils::subs("/");
  for sub in sub_root {
    let mut watcher = FsWatcher::new(idx_store.clone(), sub);
    std::thread::spawn(move || {
      watcher.start();
    });
  }
}

#[cfg(windows)]
fn win_run(idx_store: Arc<IdxStore>) {
  let drives = unsafe { get_win32_ready_drives() };
  for driv in drives {
    let mut watcher = FsWatcher::new(idx_store.clone(), driv);
    watcher.start();
  }
}
