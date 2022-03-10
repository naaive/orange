use crate::file_index::FileIndex;
use crate::file_view::FileView;
use crate::index_store::IndexStore;
use crate::KvStore;
#[cfg(target_os = "macos")]
use std::os::unix::fs::MetadataExt;
#[cfg(target_os = "linux")]
use std::os::unix::fs::MetadataExt;
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use jwalk::WalkDir;

pub struct FsWalker<'a> {
  index_store: Arc<IndexStore>,
  kv_store: Arc<KvStore<'a>>,
  root: Vec<String>,
  exclude_path: Vec<String>,
}

impl FsWalker<'_> {
  pub fn new<'a>(
    index_store: Arc<IndexStore>,
    root: Vec<String>,
    exclude_path: Vec<String>,
    kv_store: Arc<KvStore<'a>>,
  ) -> FsWalker<'a> {
    FsWalker {
      index_store,
      kv_store,
      root,
      exclude_path,
    }
  }
  pub fn start(&mut self) {
    self.do_start();
  }

  fn need_walk(&mut self, path: String) -> bool {
    let last_walk_gmt_key = "gmt_last_walk".to_string();
    let last_walk_over_key = "last_walk_over".to_string();
    let last_gmt_opt =
      self
        .kv_store
        .get_str(format!("{}#{}", last_walk_gmt_key.clone(), path.clone()));
    let curr_ts = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .ok()
      .unwrap()
      .as_secs();
    let last_over_opt =
      self
        .kv_store
        .get_str(format!("{}#{}", last_walk_over_key.clone(), path.clone()));

    match last_gmt_opt {
      None => {
        self.kv_store.put_str(
          format!("{}#{}", last_walk_gmt_key.clone(), path.clone()),
          curr_ts.to_string(),
        );
        self.kv_store.put_str(
          format!("{}#{}", last_walk_over_key.clone(), path.clone()),
          "0".to_string(),
        );
        return true;
      }
      Some(last) => {
        if last_over_opt.unwrap_or("0".to_string()).eq("0") {
          return true;
        }

        let ts = last.parse::<u64>().unwrap();
        if curr_ts - ts > 3600 * 24 * 30 {
          return true;
        }
      }
    };
    false
  }

  fn do_start(&mut self) {
    for path in self.root.clone() {
      // let walk = self.need_walk();

      self.walk_root(path.clone());
    }
  }

  fn walk_root(&mut self, path: String) {
    let walk = self.need_walk(path.clone());
    if !walk {
      return;
    }
    println!("start travel {}.", path);
    let start = SystemTime::now();

    for entry1 in WalkDir::new(path.clone())
      .into_iter()
      .filter_map(|v| v.ok())
    {

      let buf = entry1.path();
      let abs_path = buf.to_str().unwrap();
      //
      // if self
      //     .exclude_path
      //     .iter()
      //     .any(|y| abs_path.to_string().starts_with(y)) {
      //   continue
      // }

      let x1 = entry1.file_name().to_str().unwrap();
      let name = x1.to_string();
      self.index_store.add_doc(FileIndex { abs_path, name })
    }

    self.index_store.commit();
    let last_walk_over_key = "last_walk_over".to_string();
    self.kv_store.put_str(
      format!("{}#{}", last_walk_over_key.clone(), path.clone()),
      "1".to_string(),
    );
    let end = SystemTime::now();
    println!("travel {} over, cost {} s", path,end.duration_since(start).unwrap().as_secs());
  }

  fn parse_ts(time: SystemTime) -> u64 {
    let created_at = time.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
    created_at
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::data_dir;

  #[test]
  fn t1() {
    let mut index_store = Arc::new(IndexStore::new(
      "/Users/jeff/IdeaProjects/orange2/tmp/index",
    ));
    let kv_store = Arc::new(KvStore::new("/Users/jeff/IdeaProjects/orange2/tmp/kv"));
    let mut fs_walker = FsWalker::new(
      index_store.clone(),
      vec!["/Users/jeff/CLionProjects/orange2".to_string()],
      vec![],
      kv_store.clone(),
    );
    fs_walker.start();

    let mut fs_walker = FsWalker::new(
      index_store.clone(),
      vec!["/Users/jeff/Music".to_string()],
      vec![],
      kv_store,
    );
    fs_walker.start();


    // let vec1 = index_store.search("build".to_string(), 100);
    // println!("{:?}", vec1);
  }
}
