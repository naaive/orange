use crate::file_view::FileView;
use crate::{KvStore, UnitedStore};
use std::os::unix::fs::MetadataExt;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use walkdir::{WalkDir};

pub struct FsWalker<'a> {
    ustore: Arc<RwLock<UnitedStore<'a>>>,
    kv_store: KvStore<'a>,
}

impl FsWalker<'_> {
    pub fn new(ustore: Arc<RwLock<UnitedStore>>) -> FsWalker {
        // let index_writer: Arc<RwLock<UnitedStore>>
        let kv_store = KvStore::new("conf");
        FsWalker { ustore, kv_store }
    }
    pub fn start(&mut self) {
        let last_walk_gmt_key = "gmt_last_walk".to_string();
        let last_walk_over_key = "last_walk_over".to_string();
        let last_gmt_opt = self.kv_store.get_str(last_walk_gmt_key.clone());
        let curr_ts = SystemTime::now().duration_since(UNIX_EPOCH).ok().unwrap().as_secs();
        let last_over_opt = self.kv_store.get_str(last_walk_over_key.clone());

        match last_gmt_opt {
            None => {
                self.kv_store.put_str(last_walk_gmt_key, curr_ts.to_string());
                self.kv_store.put_str(last_walk_over_key, "0".to_string());
                self.do_start()
            }
            Some(last) => {
                if last_over_opt.unwrap().eq("0") {
                    self.do_start();
                    return;
                }

                let ts = last.parse::<u64>().unwrap();
                if curr_ts - ts > 3600 * 24 * 5 {
                    self.do_start()
                }
            }
        }
    }

    fn do_start(&mut self) {
        println!("start travel fs.");
        for x in WalkDir::new("/").into_iter().filter_map(|v| v.ok()) {
            std::thread::sleep(Duration::from_millis(1));
            match x.metadata() {
                Ok(meta) => {
                    let created_at = Self::parse_ts(meta.created().ok().unwrap());
                    let mod_at = Self::parse_ts(meta.modified().ok().unwrap());
                    let size = meta.size();
                    let view = FileView {
                        abs_path: x.path().to_str().unwrap().to_string(),
                        name: x.file_name().to_str().unwrap().to_string(),
                        created_at,
                        mod_at,
                        size,
                        is_dir: meta.is_dir(),
                    };
                    // println!("{:?}", view);
                    self.ustore.write().unwrap().save(view)
                }
                Err(_) => {}
            }
        }

        let last_walk_over_key = "last_walk_over".to_string();
        self.kv_store.put_str(last_walk_over_key, "1".to_string());
        println!("travel fs over.");
    }

    fn parse_ts(time: SystemTime) -> u64 {
        let created_at = time.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        created_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let mut walker = FsWalker::new(Arc::new(RwLock::new(UnitedStore::new())));
        walker.start();
    }
}
