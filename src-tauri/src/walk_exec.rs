use crate::utils;
#[cfg(windows)]
use crate::utils::get_win32_ready_drives;

use crate::idx_store::IDX_STORE;
use crate::kv_store::CONF_STORE;

use crate::walk_metrics::{WalkMatrixView, WalkMetrics};
use jwalk::{DirEntry, WalkDir};
use log::info;
use std::sync::{Arc, Mutex};

use crate::walk_metrics::WALK_METRICS;
use std::time::SystemTime;

pub fn home_dir() -> String {
  let option = dirs::home_dir();
  option.unwrap().to_str().unwrap().to_string()
}

pub unsafe fn get_walk_matrix() -> WalkMatrixView {
  WALK_METRICS
    .read()
    .unwrap()
    .view(move || IDX_STORE.num_docs())
}

use crate::user_setting::USER_SETTING;
pub fn run() {
  let home = utils::norm(&home_dir());

  start_walk_home_matrix();

  let need = need_skip_home(&home);

  if need {
    info!("skip walk home {}", home);
  } else {
    info!("start walk home {}", home);
    walk_home(&home);
  }

  end_walk_home_matrix();

  info!("start walk root {}", home);
  #[cfg(windows)]
  win_walk_root(home);

  #[cfg(unix)]
  unix_walk_root(home);
}

fn need_skip_home(home: &String) -> bool {
  let guard = USER_SETTING.read().unwrap();
  let exclude_path = guard.exclude_index_path();
  for path in exclude_path {
    if home.starts_with(path) {
      return true;
    }
  }
  return false;
}

fn end_walk_home_matrix() {
  WALK_METRICS.read().unwrap().end_home();
}

fn start_walk_home_matrix() {
  WALK_METRICS.write().unwrap().start_home();
}

#[cfg(unix)]
fn unix_walk_root(home: String) {
  let subs = utils::subs("/");
  let sz = subs.len();
  for (i, sub) in subs.iter().enumerate() {
    inc_root_walk_metrics(sz, i);

    let key = format!("walk:stat:{}", &sub);
    let opt = CONF_STORE.get_str(key.clone());
    if opt.is_some() {
      info!("{} walked", sub);
      continue;
    }
    walk(&sub, Some(home.to_string()));
    CONF_STORE.put_str(key, "1".to_string());
  }
}

fn inc_root_walk_metrics(sz: usize, i: usize) {
  WALK_METRICS
    .write()
    .unwrap()
    .root_inc_percent((i + 1) as u32, sz as u32);
}

#[cfg(windows)]
fn win_walk_root(home: String) {
  let len = win_subs_len();

  let drives = unsafe { get_win32_ready_drives() };

  let mut idx = 0;
  for mut driv in drives {
    driv = utils::norm(&driv);

    let subs = utils::subs(&driv);
    for sub in subs {
      inc_root_walk_metrics(len, idx);
      idx += 1;

      let key = format!("walk:stat:{}", &sub);
      let opt = CONF_STORE.get_str(key.clone());
      if opt.is_some() {
        info!("{} walked", sub);
        continue;
      }

      walk(&sub, Some(home.to_string()));
      CONF_STORE.put_str(key, "1".to_string());
    }
  }
}

#[cfg(windows)]
fn win_subs_len() -> usize {
  let drives = unsafe { get_win32_ready_drives() };
  let mut sz = 0;
  for mut driv in drives {
    driv = utils::norm(&driv);
    let subs = utils::subs(&driv);
    sz += subs.len();
  }
  sz
}

fn walk_home(home: &String) {
  let key = format!("walk:stat:{}", home);
  let opt = CONF_STORE.get_str(key.clone());
  if opt.is_some() {
    info!("home walked {}", home);
    return;
  }

  let home_name = utils::path2name(home.to_string());
  IDX_STORE.add(home_name, home.clone().to_string(), true, "".to_string());
  walk(&home, None);
  CONF_STORE.put_str(key, "1".to_string());
}

fn walk(path: &String, skip_path_opt: Option<String>) {
  let start = SystemTime::now();
  info!("start travel {}", path);
  let mut cnt = 0;

  let mut generic = WalkDir::new(&path);
  if skip_path_opt.is_some() {
    let skip_path = skip_path_opt.unwrap();
    let home_name = utils::path2name(skip_path.clone());
    generic = generic.process_read_dir(move |_depth, _path, _read_dir_state, children| {
      children.iter_mut().for_each(|dir_entry_result| {
        if let Ok(dir_entry) = dir_entry_result {
          let curr_path = utils::norm(dir_entry.path().to_str().unwrap_or(""));

          let guard = USER_SETTING.read().unwrap();
          let exclude_path = guard.exclude_index_path();

          if curr_path.eq(skip_path.as_str())
            || curr_path.eq("/proc")
            || exclude_path.iter().any(|x| curr_path.starts_with(x))
            || curr_path.eq(&format!("/System/Volumes/Data/Users/{}", home_name))
          {
            info!("skip path {}", curr_path);
            dir_entry.read_children_path = None;
          }
        }
      });
    });
  }

  for entry in generic {
    cnt += 1;
    if entry.is_err() {
      continue;
    }
    let en: DirEntry<((), ())> = entry.unwrap();
    let buf = en.path();
    let file_type = en.file_type();
    let is_dir = file_type.is_dir();
    let path = buf.to_str().unwrap();
    let name = en.file_name().to_str().unwrap();
    let ext = utils::file_ext(name);
    IDX_STORE.add(name.to_string(), path.to_string(), is_dir, ext.to_string());
  }
  let end = SystemTime::now();
  IDX_STORE.commit();
  info!(
    "cost {} s, total {} files",
    end.duration_since(start).unwrap().as_secs(),
    cnt
  );
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::kv_store::KvStore;
  use std::time::UNIX_EPOCH;

  #[test]
  fn t1() {
    let dir = utils::data_dir();

    let string = format!("{}/orangecachedata", dir);
    println!("{}", string);
    let _dir_all = std::fs::remove_dir_all(string);
    utils::init_log();

    let dir = utils::data_dir();
    let _conf_path = format!("{}{}", dir, "/orangecachedata/conf");
    let _idx_path = format!("{}{}", dir, "/orangecachedata/idx");

    run();
    IDX_STORE.commit();
  }

  #[test]
  fn disable_walk() {
    utils::init_log();

    let dir = utils::data_dir();
    let conf_path = format!("{}{}", dir, "/orangecachedata/conf");
    let conf_store = Arc::new(KvStore::new(&conf_path));
    let curr_ts = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs();
    conf_store.put_str("version".to_string(), "0.3.0".to_string());
    conf_store.put_str("last_index_ts".to_string(), curr_ts.to_string());
  }
}
