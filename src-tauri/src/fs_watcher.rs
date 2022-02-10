extern crate notify;
#[cfg(target_os = "windows")]
use {
    std::os::windows::fs::MetadataExt
};
#[cfg(target_os = "linux")]
use {
    std::os::unix::fs::MetadataExt
};
#[cfg(target_os = "macos")]
use {
    std::os::unix::fs::MetadataExt
};
use crate::file_view::FileView;
use crate::{UnitedStore};
use notify::{raw_watcher, Op, RawEvent, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

pub struct FsWatcher<'a> {
    ustore: Arc<RwLock<UnitedStore<'a>>>,
    path:String
}

impl FsWatcher<'_> {
    pub fn new<'a>(ustore: Arc<RwLock<UnitedStore<'a>>>, path: String) -> FsWatcher<'a> {
        FsWatcher { ustore, path: path }
    }

    pub fn start(&mut self) {
        let (tx, rx) = channel();
        let mut watcher = raw_watcher(tx).unwrap();
        watcher.watch(self.path.as_str(), RecursiveMode::Recursive).unwrap();

        loop {
            match rx.recv() {
                Ok(RawEvent {
                    path: Some(path),
                    op: Ok(op),
                    cookie,
                }) => {
                    let result = path.metadata();
                    match result {
                        Ok(meta) => {
                            // println!("{:?} {:?} ({:?})", op, path, cookie);
                            let abs_path = path.to_str().unwrap().to_string();
                            let name = path
                                .file_name()
                                .map(|x| x.to_str().unwrap())
                                .unwrap_or_default()
                                .to_string();


                            let created_at = Self::parse_ts(meta.created().unwrap());
                            let mod_at = Self::parse_ts(meta.modified().unwrap());


                            #[cfg(windows)]
                            let size = meta.file_size();
                            #[cfg(unix)]
                            let size = meta.size();


                            if Op::REMOVE == op {
                                self.ustore.write().unwrap().del(&abs_path);
                            } else {
                                self.ustore.write().unwrap().save(FileView {
                                    abs_path,
                                    name,
                                    created_at,
                                    mod_at,
                                    size,
                                    // is_symbol,
                                    is_dir: meta.is_dir()
                                });
                            }
                        }
                        Err(_) => {}
                    }
                }
                Ok(event) => println!("broken event: {:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    }

    fn parse_ts(time: SystemTime) -> u64 {
        let created_at = time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        created_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let store = UnitedStore::new();
        let mut watcher = FsWatcher::new(Arc::new(RwLock::new(store)), "".to_string());
        watcher.start();
    }
}
