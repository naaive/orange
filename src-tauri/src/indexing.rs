use crate::{utils, walk_exec, watch_exec, CONF_STORE, IDX_STORE};
use log::{error, info};
#[cfg(windows)]
use std::sync::mpsc;
#[cfg(windows)]
use std::sync::mpsc::Sender;
use std::sync::Arc;
#[cfg(windows)]
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::kv_store::KvStore;

use crate::idx_store::IdxStore;

#[cfg(windows)]
use crate::usn_journal_watcher::Watcher;

const STORE_PATH: &'static str = "orangecachedata";

#[cfg(windows)]
const RECYCLE_PATH: &'static str = "$RECYCLE.BIN";
const VERSION: &'static str = "0.0.7";
const LAST_INDEX_TS: &'static str = "last_index_ts";

pub fn run() {
  let conf_path = format!("{}{}", utils::data_dir(), "/orangecachedata/conf");
  let idx_path = format!("{}{}", utils::data_dir(), "/orangecachedata/idx");

  let conf_store = Arc::new(KvStore::new(&conf_path));
  housekeeping(conf_store.clone());

  let idx_store = Arc::new(IdxStore::new(&idx_path));

  unsafe {
    IDX_STORE = Some(idx_store.clone());
  }
  unsafe {
    CONF_STORE = Some(conf_store.clone());
  }

  let reindex = need_reindex(conf_store.clone());
  info!("need reindex: {}", reindex);
  if reindex {
    let conf_store_bro = conf_store.clone();
    let idx_store_bro = idx_store.clone();
    walk_exec::run(conf_store_bro, idx_store_bro);
    conf_store.put_str(LAST_INDEX_TS.to_string(), curr_ts().to_string());
    info!("walk exec done");
  };

  let idx_store_bro = idx_store.clone();
  idx_store_bro.disable_full_indexing();

  info!("start fs watch");
  #[cfg(windows)]
  win_watch(idx_store_bro);

  #[cfg(unix)]
  watch_exec::run(idx_store_bro);
}

#[cfg(windows)]
fn win_watch(idx_store_bro: Arc<IdxStore>) {
  let success = unsafe { maybe_usn_watch() };
  if success {
    info!("usn success");
  } else {
    info!("usn err, use watch");
    watch_exec::run(idx_store_bro);
  }
}

fn need_reindex(kv_store: Arc<KvStore>) -> bool {
  let key = LAST_INDEX_TS.to_string();

  return match kv_store.get_str(key.clone()) {
    None => true,
    Some(val) => {
      let ts = val.parse::<u64>().unwrap();
      if curr_ts() - ts > 3600 * 24 * 30 {
        return true;
      }
      false
    }
  };
}

fn curr_ts() -> u64 {
  let curr_ts = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs();
  curr_ts
}

fn housekeeping(kv_store: Arc<KvStore>) {
  info!("housekeeping...");
  let version_opt = kv_store.get_str("version".to_string());
  match version_opt {
    None => {
      let _ = std::fs::remove_dir_all(&format!("{}{}", utils::data_dir(), "/orangecachedata/idx"));
      kv_store.clear();
      kv_store.put_str("version".to_string(), VERSION.to_string());
      info!("init version {}", VERSION);
    }
    Some(version) => {
      if !version.eq(VERSION) {
        let _ =
          std::fs::remove_dir_all(&format!("{}{}", utils::data_dir(), "/orangecachedata/idx"))
            .unwrap();
        kv_store.clear();
        kv_store.put_str("version".to_string(), VERSION.to_string());
        info!("clean old version cachedata");
      } else {
        info!("no need to clean, current version:{}", VERSION);
      }
    }
  }
}

#[cfg(windows)]
unsafe fn maybe_usn_watch() -> bool {
  let (tx, rx) = mpsc::channel();
  let nos = utils::get_win32_ready_drive_nos();

  for no in nos {
    let volume_path = utils::build_volume_path(no.as_str());
    let tx_clone = tx.clone();
    start_usn_watch(no, volume_path, tx_clone);
  }

  let success = rx.recv().unwrap();
  success
}

#[cfg(windows)]
unsafe fn start_usn_watch<'a>(no: String, volume_path: String, tx_clone: Sender<bool>) {
  info!("start_usn_watch {}", volume_path);

  std::thread::spawn(move || {
    let kv_store = CONF_STORE.clone().unwrap();
    let key = format!("usn#next_usn#{}", volume_path.clone());
    let next_usn = kv_store
      .get_str(key.clone())
      .unwrap_or("0".to_string())
      .parse()
      .unwrap();

    let result = Watcher::new(volume_path.as_str(), None, Some(next_usn));
    if result.is_err() {
      error!(" {:?} ", result.err());
      let _ = tx_clone.send(false);
      return;
    }

    let mut watcher = result.unwrap();
    let _ = tx_clone.send(true);
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
          info!("volume {} usn history loaded", volume_path);
        }
        std::thread::sleep(Duration::from_millis(500));
      } else {
        let usn_no = records.last().unwrap().usn.to_string();

        for record in records {
          let path = record.path.to_str().unwrap();
          let file_name = record.file_name;
          let abs_path = format!("{}:{}", no.as_str(), path);

          if abs_path.contains(STORE_PATH) || abs_path.contains(RECYCLE_PATH) {
            continue;
          }

          IDX_STORE
            .clone()
            .unwrap()
            .add(&file_name, &abs_path.clone())
        }

        kv_store.put_str(key.clone(), usn_no);
      }
    }
  });
}
