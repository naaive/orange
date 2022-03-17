use crate::idx_store::IdxStore;
use crate::kv_store::KvStore;

use crate::utils;
#[cfg(windows)]
use crate::utils::get_win32_ready_drives;

use jwalk::WalkDir;
use log::info;
use std::sync::Arc;
use std::time::SystemTime;

pub fn home_dir() -> String {
  let option = dirs::home_dir();
  option.unwrap().to_str().unwrap().to_string()
}

pub fn run(conf_store: Arc<KvStore>, idx_store: Arc<IdxStore>) {
  let home = utils::norm(&home_dir());

  info!("start walk home {}", home);
  walk_home(conf_store.clone(), idx_store.clone(), &home);

  info!("start walk root {}", home);
  #[cfg(windows)]
  win_walk_root(conf_store, idx_store, home);

  #[cfg(unix)]
  unix_walk_root(conf_store, idx_store, home);
}

#[cfg(unix)]
fn unix_walk_root(conf_store: Arc<KvStore>, idx_store: Arc<IdxStore>, home: String) {
  let subs = utils::subs("/");
  for sub in subs {
    let key = format!("walk:stat:{}", &sub);
    let opt = conf_store.get_str(key.clone());
    if opt.is_some() {
      info!("{} walked", sub);
      continue;
    }
    walk(idx_store.clone(), &sub, Some(home.to_string()));
    conf_store.put_str(key, "1".to_string());
  }
}

#[cfg(windows)]
fn win_walk_root(conf_store: Arc<KvStore>, idx_store: Arc<IdxStore>, home: String) {
  let drives = unsafe { get_win32_ready_drives() };

  for mut driv in drives {
    driv = utils::norm(&driv);

    let subs = utils::subs(&driv);
    for sub in subs {
      let key = format!("walk:stat:{}", &sub);
      let opt = conf_store.get_str(key.clone());
      if opt.is_some() {
        info!("{} walked", sub);
        continue;
      }

      walk(idx_store.clone(), &sub, Some(home.to_string()));
      conf_store.put_str(key, "1".to_string());
    }
  }
}

fn walk_home(conf_store: Arc<KvStore>, idx_store: Arc<IdxStore>, home: &String) {
  let key = format!("walk:stat:{}", home);
  let opt = conf_store.get_str(key.clone());
  if opt.is_some() {
    info!("home walked {}", home);
    return;
  }

  let home_name = utils::path2name(home.as_str().to_string()).unwrap_or("".to_string());
  idx_store.add(&home_name, &home);
  walk(idx_store, &home, None);
  conf_store.put_str(key, "1".to_string());
}

fn walk(store: Arc<IdxStore>, path: &String, skip_path_opt: Option<String>) {
  let start = SystemTime::now();
  info!("start travel {}", path);
  let mut cnt = 0;

  let mut generic = WalkDir::new(&path);
  if skip_path_opt.is_some() {
    let skip_path = skip_path_opt.unwrap();
    let home_name = utils::path2name(skip_path.clone()).unwrap_or("".to_string());
    generic = generic.process_read_dir(move |_depth, _path, _read_dir_state, children| {
      children.iter_mut().for_each(|dir_entry_result| {
        if let Ok(dir_entry) = dir_entry_result {
          let curr_path = utils::norm(dir_entry.path().to_str().unwrap_or(""));
          if curr_path.eq(skip_path.as_str())
            || curr_path.eq("/proc")
            || curr_path.eq(&format!("/System/Volumes/Data/Users/{}", home_name))
          {
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
    let en = entry.unwrap();
    let buf = en.path();
    let path = buf.to_str().unwrap();
    let name = en.file_name().to_str().unwrap();

    store.add(name, path);
  }
  let end = SystemTime::now();
  store.commit();
  info!(
    "cost {} s, total {} files",
    end.duration_since(start).unwrap().as_secs(),
    cnt
  );
}
#[test]
fn t1() {
  use crate::utils::init_log;
  init_log();

  let conf_path = format!("{}{}", utils::data_dir(), "/orangecachedata/conf");
  let idx_path = format!("{}{}", utils::data_dir(), "/orangecachedata/idx");

  let conf_store = Arc::new(KvStore::new(&conf_path));
  let idx_store = Arc::new(IdxStore::new(&idx_path));

  run(conf_store, idx_store);
}
