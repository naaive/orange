#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crate::idx_store::IdxStore;
use crate::kv_store::KvStore;
use crate::user_setting::{Theme, UserSetting};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;

mod file_doc;
mod file_view;
mod fs_watcher;
mod idx_store;
mod indexing;
mod kv_store;
mod ui;
mod user_setting;
#[cfg(windows)]
mod usn_journal_watcher;
mod utils;
mod walk_exec;
mod walk_metrics;
mod watch_exec;
#[macro_use]
extern crate lazy_static;

fn main() {
  utils::init_log();
  indexing::run();
  ui::show();
}
