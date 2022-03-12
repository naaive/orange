use crate::fs_watcher::FsWatcher;
use crate::idx_store::IdxStore;
use crate::utils::get_win32_ready_drives;
use std::sync::Arc;

pub fn run(idx_store: Arc<IdxStore>) {
    let drives = unsafe { get_win32_ready_drives() };
    for driv in drives {
        FsWatcher::new(idx_store.clone(), driv);
    }
}
