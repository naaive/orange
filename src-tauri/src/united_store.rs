// use crate::file_index::FileIndex;
// use crate::file_kv::FileKv;
// use crate::file_view::FileView;
// use crate::kv_store::KvStore;
// use crate::pinyin_tokenizer::tokenize;
// use crate::{IndexStore, utils};
// use sha256::{digest, digest_bytes};
// use std::collections::HashSet;
// use std::fs;
// use std::iter::FromIterator;
// use std::os::unix::fs::MetadataExt;
// use std::path::Path;
// use std::time::SystemTime;
// use crate::utils::data_dir;
//
// #[derive(Clone)]
// pub struct UnitedStore<'a> {
//   kv: KvStore<'a>,
//   idx: Box<IndexStore>,
// }
//
// impl UnitedStore<'_> {
//   pub fn new<'a>() -> UnitedStore<'a> {
//     let kv_path = &format!("{}{}", data_dir(), "/orangecachedata/kv");
//     let kv = KvStore::new(kv_path);
//     let idx_path = &format!("{}{}", data_dir(), "/orangecachedata/idx");
//     let idx = IndexStore::new(idx_path);
//     let x = Box::new(idx);
//     UnitedStore { kv, idx:x }
//   }
//
//
//
//
//   pub fn search(&self, kw: &str, limit: usize) -> Vec<FileView> {
//     let mut file_views = Vec::new();
//     let arr = self.idx.search(String::from(kw), limit);
//
//   }
// }
//
// #[cfg(test)]
// mod tests {
//   use super::*;
//
//   #[test]
//   fn t1() {
//
//   }
//
//   #[test]
//   fn t2() {
//     let store = UnitedStore::new();
//     let x = store.search("usr", 100);
//     println!("{:?}", x);
//   }
//
//   #[test]
//   fn t3() {
//     let test_str = "übercode"; // type &str
//
//     let uppercase_test_string = test_str.to_uppercase(); // type String
//
//     let uppercase_test_str = uppercase_test_string.as_str(); // back to type &str
//
//     println! {"{}", test_str};
//     println! {"{:?}", uppercase_test_string};
//     println! {"{}", uppercase_test_str};
//   }
// }
