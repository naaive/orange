use core::fmt;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::RwLock;

use log::set_logger_racy;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

use crate::kv_store::KvStore;
use crate::utils;

lazy_static! {
  pub static ref USER_SETTING: RwLock<UserSetting> = RwLock::new(UserSetting::default());
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Theme {
  Light,
  Dark,
}

#[derive(Debug)]
struct UserSettingError {
  details: String,
}

impl UserSettingError {
  pub fn new(details: String) -> Self {
    UserSettingError { details }
  }
}

impl Display for UserSettingError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.details)
  }
}

impl Error for UserSettingError {
  fn description(&self) -> &str {
    self.details.as_str()
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSetting {
  theme: Theme,
  exclude_index_path: Vec<String>,
}

impl UserSetting {
  pub fn theme(&self) -> &Theme {
    &self.theme
  }
  pub fn exclude_index_path(&self) -> &Vec<String> {
    &self.exclude_index_path
  }
}

impl UserSetting {
  pub fn set_theme(&mut self, theme: Theme) {
    self.theme = theme;
    self.store();
  }
  pub fn set_exclude_index_path(
    &mut self,
    exclude_index_path: Vec<String>,
  ) -> std::result::Result<(), UserSettingError> {
    for path in &exclude_index_path {
      if std::fs::metadata(path).is_err() {
        return Err(UserSettingError::new(path.to_string()));
      }
    }
    self.exclude_index_path = exclude_index_path;
    self.store();
    Ok(())
  }
}

const PREFERENCE_FILE: &'static str = "/preference.json";

impl UserSetting {
  fn store(&self) {
    let path = UserSetting::build_conf_path();
    let contents = serde_json::to_string_pretty(self).unwrap();
    let _ = std::fs::write(path, contents);
  }

  fn load() -> std::result::Result<UserSetting, Box<dyn std::error::Error>> {
    let path = UserSetting::build_conf_path();
    let string = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&string).expect("deserialize error"))
  }

  fn build_conf_path() -> String {
    let path = format!("{}{}", utils::data_dir(), PREFERENCE_FILE);
    path
  }
}

impl Default for UserSetting {
  fn default() -> Self {
    UserSetting::load().unwrap_or_else(|_| {
      let setting = UserSetting {
        theme: Theme::Light,
        exclude_index_path: vec![],
      };
      setting.store();
      setting
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn t1() {
    let mut setting = UserSetting::default();
    // setting.set_theme(Theme::Dark);
    println!("{:?}", setting);
  }
}
