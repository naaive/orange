use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct FileIndex<'a> {
  pub abs_path: &'a str,
  pub name: String,
}

#[test]
fn t1() {}
