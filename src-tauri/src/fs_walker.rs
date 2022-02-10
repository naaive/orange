use crate::file_view::FileView;
use crate::{KvStore, UnitedStore};
#[cfg(target_os = "macos")]
use std::os::unix::fs::MetadataExt;
#[cfg(target_os = "linux")]
use std::os::unix::fs::MetadataExt;
#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

pub struct FsWalker<'a> {
  ustore: Arc<RwLock<UnitedStore<'a>>>,
  kv_store: KvStore<'a>,
  root: Vec<String>,
  exclude_path: Vec<String>,
}

impl FsWalker<'_> {
  pub fn new(
    ustore: Arc<RwLock<UnitedStore>>,
    root: Vec<String>,
    exclude_path: Vec<String>,
  ) -> FsWalker {
    // let index_writer: Arc<RwLock<UnitedStore>>
    let kv_store = KvStore::new("./cachedata/conf");
    FsWalker {
      ustore,
      kv_store,
      root,
      exclude_path,
    }
  }
  pub fn start(&mut self) {
    self.do_start()
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
        if last_over_opt.unwrap().eq("0") {
          return true;
        }

        let ts = last.parse::<u64>().unwrap();
        if curr_ts - ts > 3600 * 24 * 5 {
          return true;
        }
      }
    };
    false
  }

  fn do_start(&mut self) {
    println!("start travel fs.");

    for path in self.root.clone() {
      // let walk = self.need_walk();

      self.walk_root(path.clone());
    }

    println!("travel fs over.");
  }

  fn walk_root(&mut self, path: String) {
    let walk = self.need_walk(path.clone());
    if !walk {
      return;
    }

    for x in WalkDir::new(path.clone())
      .into_iter()
      .filter_entry(|x| {
        !self
          .exclude_path
          .iter()
          .any(|y| x.path().to_str().unwrap().to_string().starts_with(y))
      })
      .filter_map(|v| v.ok())
    {
      std::thread::sleep(Duration::from_millis(1));
      match x.metadata() {
        Ok(meta) => {
          let created_at = Self::parse_ts(meta.created().ok().unwrap());
          let mod_at = Self::parse_ts(meta.modified().ok().unwrap());
          #[cfg(windows)]
          let size = meta.file_size();
          #[cfg(unix)]
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
    self.kv_store.put_str(
      format!("{}#{}", last_walk_over_key.clone(), path.clone()),
      "1".to_string(),
    );
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
    let mut walker = FsWalker::new(
      Arc::new(RwLock::new(UnitedStore::new())),
      vec!["".to_string()],
      vec![],
    );
    walker.start();
  }

  #[test]
  fn t2() {
    let vec1 = vec!["hi", "jeff", "sam"];
    let x1 = vec1.iter().any(|x| x.starts_with("w"));
    println!("{}", x1);
  }

  #[test]
  fn t3() {
    let owned_string = "hi".to_string();
    let jack = "another_owned_string".to_string();
    let string = format!("{}{}", owned_string, jack);
    println!("{}", string);
  }
}
