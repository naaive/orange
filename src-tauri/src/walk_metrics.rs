use crate::IdxStore;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct WalkMetrics {
  percent: Arc<RwLock<u32>>,
  home_over: Arc<AtomicBool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WalkMatrixView {
  percent: u32,
  total_files: u64,
}

impl Default for WalkMatrixView {
  fn default() -> Self {
    WalkMatrixView {
      percent: 0,
      total_files: 0,
    }
  }
}

impl WalkMatrixView {
  pub fn new(percent: u32, total_files: u64) -> Self {
    WalkMatrixView {
      percent,
      total_files,
    }
  }
}

impl WalkMetrics {
  pub fn new(percent: u32) -> Self {
    let percent = Arc::new(RwLock::new(percent));
    WalkMetrics {
      percent,
      home_over: Arc::new(AtomicBool::new(false)),
    }
  }

  pub fn view<F>(&self, curr_num_docs: F) -> WalkMatrixView
  where
    F: Send + 'static,
    F: Fn() -> u64,
  {
    WalkMatrixView::new(*self.percent.read().unwrap(), curr_num_docs())
  }

  pub fn start_home(&mut self) {
    let home_over0 = self.home_over.clone();
    let percent0 = self.percent.clone();
    thread::spawn(move || loop {
      if home_over0.load(Ordering::Relaxed) {
        break;
      }
      if *percent0.read().unwrap() < 30 {
        *percent0.write().unwrap() += 1;
      }
      thread::sleep(Duration::from_secs(1));
    });
  }

  pub fn end_home(&self) {
    self.home_over.store(true, Ordering::Relaxed);
  }

  pub fn root_inc_percent(&mut self, walked_dir: u32, total_dir: u32, total_files: u64) {
    *self.percent.write().unwrap() = ((walked_dir * 70 / total_dir) + 30);
  }
}

impl Default for WalkMetrics {
  fn default() -> Self {
    WalkMetrics::new(0)
  }
}

#[cfg(test)]
mod tests {}
