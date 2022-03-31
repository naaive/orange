use log::set_logger_racy;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

use crate::kv_store::KvStore;
use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub enum Theme {
    Light,
    Dark,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserConf {
    theme: Theme,
    exclude_index_path: Vec<String>,

}

impl Default for UserConf {
    fn default() -> Self {
        UserConf{ theme: Theme::Light, exclude_index_path: vec![] }
    }
}

impl UserConf {
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }
    pub fn set_exclude_index_path(&mut self, exclude_index_path: Vec<String>) {
        self.exclude_index_path = exclude_index_path;
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }
    pub fn exclude_index_path(&self) -> &Vec<String> {
        &self.exclude_index_path
    }
}

pub struct UserSetting<'a> {
    store: KvStore<'a>,
    user_conf: UserConf,
}

impl UserSetting<'_> {
    pub fn set_theme(&mut self, theme: Theme) {
        self.user_conf.set_theme(theme);
        self.save();
    }
    pub fn set_exclude_index_path(&mut self, exclude_index_path: Vec<String>) {
        self.user_conf.set_exclude_index_path(exclude_index_path);
        self.save();
    }
    pub fn theme(&self) -> &Theme {
        &self.user_conf.theme
    }
    pub fn exclude_index_path(&self) -> &Vec<String> {
        &self.user_conf.exclude_index_path
    }
    fn save(&self) {
        self.store.put_str("user_setting".to_string(), serde_json::to_string(&self.user_conf).unwrap())
    }
}

impl Default for UserSetting<'_> {
    fn default() -> Self {
        let dir = utils::data_dir();
        let user_setting = format!("{}{}", dir, "/orangecachedata/user_setting");
        let store = KvStore::new(&user_setting);

        return match store.get_str("user_setting".to_string()) {
            None => {
                let user_conf = UserConf::default();
                let setting = UserSetting {
                    store,
                    user_conf
                };
                setting.save();
                setting
            }
            Some(x) => {
                let  user_conf: UserConf = serde_json::from_str(&x).unwrap();
                UserSetting{ store, user_conf }
            }
        };

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t2() {
        let mut setting = UserSetting::default();
        setting.set_theme(Theme::Dark);
    }
    #[test]
    fn t1() {
        let setting = UserSetting::default();
        println!("{:?}", setting.user_conf);
    }
}




