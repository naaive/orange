use crate::idx_store::IdxStore;
use crate::kv_store::KvStore;
use crate::file_doc::FileDoc;
use std::ops::Deref;

#[cfg(windows)]
use crate::utils::get_win32_ready_drives;
use crate::{utils, IDX_STORE};

use crate::walk_metrics::{WalkMatrixView, WalkMetrics};
use jwalk::{DirEntry, WalkDir};
use log::info;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, SystemTime};


static mut WALK_METRICS: Option<Arc<Mutex<WalkMetrics>>> = None;

pub fn home_dir() -> String {
  let option = dirs::home_dir();
  option.unwrap().to_str().unwrap().to_string()
}

pub unsafe fn get_walk_matrix() -> WalkMatrixView {
  let idx_store = IDX_STORE.clone();
  if idx_store.is_none() {
    return WalkMatrixView::default();
  }
  if WALK_METRICS.is_none() && !idx_store.clone().unwrap().is_full_indexing() {
    return WalkMatrixView::new(100, idx_store.clone().unwrap().num_docs());
  }
  WALK_METRICS
    .clone()
    .unwrap()
    .lock()
    .unwrap()
    .view(move || idx_store.clone().unwrap().num_docs())
}

pub fn run(conf_store: Arc<KvStore>, idx_store: Arc<IdxStore>) {
  init_walk_matrix();
  let home = utils::norm(&home_dir());

  start_walk_home_matrix();

  info!("start walk home {}", home);
  walk_home(conf_store.clone(), idx_store.clone(), &home);

  end_walk_home_matrix();

  info!("start walk root {}", home);
  #[cfg(windows)]
  win_walk_root(conf_store, idx_store, home);

  #[cfg(unix)]
  unix_walk_root(conf_store, idx_store, home);
}

fn end_walk_home_matrix() {
  unsafe {
    let mut walk_matrix0 = WALK_METRICS.clone().unwrap();
    walk_matrix0.lock().unwrap().end_home();
  }
}

fn start_walk_home_matrix() {
  unsafe {
    let mut walk_matrix0 = WALK_METRICS.clone().unwrap();
    walk_matrix0.lock().unwrap().start_home();
  }
}

fn init_walk_matrix() {
  unsafe {
    WALK_METRICS = Some(Arc::new(Mutex::new(WalkMetrics::default())));
  }
}

#[cfg(unix)]
fn unix_walk_root(conf_store: Arc<KvStore>, idx_store: Arc<IdxStore>, home: String) {
  let subs = utils::subs("/");
  let sz = subs.len();
  for (i, sub) in subs.iter().enumerate() {
    inc_root_walk_metrics(sz, i);

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

fn inc_root_walk_metrics(sz: usize, i: usize) {
  unsafe {
    WALK_METRICS
      .clone()
      .unwrap()
      .lock()
      .unwrap()
      .root_inc_percent((i + 1) as u32, sz as u32);
  }
}

#[cfg(windows)]
fn win_walk_root(conf_store: Arc<KvStore>, idx_store: Arc<IdxStore>, home: String) {
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

fn walk_home(conf_store: Arc<KvStore>, idx_store: Arc<IdxStore>, home: &String) {
  let key = format!("walk:stat:{}", home);
  let opt = conf_store.get_str(key.clone());
  if opt.is_some() {
    info!("home walked {}", home);
    return;
  }

  let home_name = utils::path2name(home.as_str().to_string()).unwrap_or("".to_string());
  // idx_store.add(&home_name, &home);
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
    let en: DirEntry<((), ())> = entry.unwrap();
    let doc = FileDoc::from(en);
    store.add(doc);
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
