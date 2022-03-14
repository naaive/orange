
use crate::idx_store::IdxStore;
#[cfg(windows)]
use crate::utils::get_win32_ready_drives;


use std::sync::Arc;
use crate::fs_watcher::FsWatcher;


pub fn run(idx_store: Arc<IdxStore>) {
  #[cfg(windows)]
  win_run(idx_store);

  #[cfg(linux)]
  linux_run(idx_store);

  #[cfg(macos)]
  macos_run(idx_store);
}

#[cfg(linux)]
fn macos_run(idx_store: Arc<IdxStore>) {
  let mut watcher = FsWatcher::new(idx_store.clone(), "/");
  thread::spawn(move || {
    watcher.start();
  });
}

#[cfg(linux)]
fn linux_run(idx_store: Arc<IdxStore>) {
  let sub_root = utils::subs("/");
  for sub in sub_root {
    let mut watcher = FsWatcher::new(idx_store.clone(), sub);
    thread::spawn(move || {
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
