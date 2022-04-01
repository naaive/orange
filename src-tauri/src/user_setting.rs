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

#[derive(Debug)]
pub struct UserSettingError {
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
  theme: String,
  lang: String,
  exclude_index_path: Vec<String>,
}

impl UserSetting {
  pub fn lang(&self) -> &str {
    &self.lang
  }
  pub fn theme(&self) -> &String {
    &self.theme
  }
  pub fn exclude_index_path(&self) -> &Vec<String> {
    &self.exclude_index_path
  }
}

impl UserSetting {
  pub fn set_lang(&mut self, lang: String) {
    self.lang = lang;
    self.store();
  }
  pub fn set_theme(&mut self, theme: String) {
    self.theme = theme;
    self.store();
  }
  pub fn add_exclude_index_path(
    &mut self,
    path: String,
  ) -> std::result::Result<(), UserSettingError> {
    if std::fs::metadata(&path).is_err() {
      return Err(UserSettingError::new(path.to_string()));
    }
    self.exclude_index_path.push(path);
    self.store();
    Ok(())
  }

  pub fn remove_exclude_index_path(&mut self, path: String) {
    self.set_exclude_index_path(
      self
        .exclude_index_path
        .iter()
        .filter(|x| !x.eq(&path))
        .collect(),
    );
    self.store();
  }

  pub fn set_exclude_index_path(&mut self, exclude_index_path: Vec<String>) {
    self.exclude_index_path = exclude_index_path;
  }
}

const PREFERENCE_FILE: &'static str = "/preference.json";

impl UserSetting {
  fn store(&self) {
    let path = UserSetting::build_conf_path();
    let contents = serde_json::to_string_pretty(self).unwrap();
    let _ = std::fs::write(path, contents);
  }

  fn load() -> std::result::Result<UserSetting, Box<dyn Error>> {
    let path = UserSetting::build_conf_path();
    let string = std::fs::read_to_string(path)?;
    let result1: Result<UserSetting> = serde_json::from_str(&string);
    result1.map_err(|x| x.to_string().into())
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
        theme: "light".to_string(),
        lang: "en".to_string(),
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
    // let string = "zh".into();
    // setting.set_lang(string);
    setting.set_theme("dark".to_string());
    println!("{:?}", setting);
  }
}
