use crate::idx_store::IDX_STORE;
use crate::kv_store::CONF_STORE;
use crate::{utils, walk_exec, watch_exec};
use log::info;
use crate::walk_metrics::WALK_METRICS;

#[cfg(windows)]
use log::error;
#[cfg(windows)]
use std::sync::mpsc;
#[cfg(windows)]
use std::sync::mpsc::Sender;

#[cfg(windows)]
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

#[cfg(windows)]
use crate::usn_journal_watcher::Watcher;

#[cfg(windows)]
const STORE_PATH: &'static str = "orangecachedata";
#[cfg(windows)]
const RECYCLE_PATH: &'static str = "$RECYCLE.BIN";
const VERSION: &'static str = "0.6.0";
const LAST_INDEX_TS: &'static str = "last_index_ts";

pub fn run() {
  std::thread::spawn(|| {
    do_run();
  });
}

fn do_run() {
  housekeeping();

  let reindex = need_reindex();
  info!("need reindex: {}", reindex);
  if reindex {
    walk_exec::run();
    CONF_STORE.put_str(LAST_INDEX_TS.to_string(), curr_ts().to_string());
    info!("walk exec done");
  };

  IDX_STORE.disable_full_indexing();
  WALK_METRICS.write().unwrap().end_of_no_reindex();
  info!("start fs watch");

  #[cfg(windows)]
  if cfg!(target_os = "windows") {
    if reindex {
      info!("use watcher due to reindex");
      watch_exec::run();
    } else {
      info!("try use usn");
      win_watch();
    }
  }
  #[cfg(unix)]
  watch_exec::run();
}

#[cfg(windows)]
fn win_watch() {
  let success = unsafe { maybe_usn_watch() };
  if success {
    info!("usn success");
  } else {
    info!("usn err, use watch");
    watch_exec::run();
  }
}

pub fn reindex() {
  CONF_STORE.put_str("reindex".to_string(), "1".to_string());
}

fn need_reindex() -> bool {
  let key = LAST_INDEX_TS.to_string();

  return match CONF_STORE.get_str(key.clone()) {
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

pub fn housekeeping() {
  info!("housekeeping...");

  let reidx_opt = CONF_STORE.get_str("reindex".to_string());
  match reidx_opt {
    None => {
      info!("no need to reindex");
    }
    Some(_) => {
      clear();
      info!("detect reindex sign");
      return;
    }
  }

  let version_opt = CONF_STORE.get_str("version".to_string());
  match version_opt {
    None => {
      clear();
      info!("init version {}", VERSION);
    }
    Some(version) => {
      if !version.eq(VERSION) {
        clear();
        info!("clean old version cachedata");
      } else {
        info!("no need to clean, current version:{}", VERSION);
      }
    }
  }
}

fn clear() {
  let _ = std::fs::remove_dir_all(&format!("{}{}", utils::data_dir(), "/orangecachedata/idx"));
  CONF_STORE.clear();
  CONF_STORE.put_str("version".to_string(), VERSION.to_string());
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
    let key = format!("usn#next_usn#{}", volume_path.clone());
    let next_usn = CONF_STORE
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

          let is_dir = std::fs::metadata(abs_path.clone())
            .map(|x| x.is_dir())
            .unwrap_or(false);
          let name0 = file_name.clone();
          let ext = utils::file_ext(&name0);

          IDX_STORE.add(file_name, abs_path.clone(), is_dir, ext.to_string());
        }

        CONF_STORE.put_str(key.clone(), usn_no);
      }
    }
  });
}
