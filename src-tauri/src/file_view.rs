use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileView {
  pub abs_path: String,
  pub name: String,
  pub created_at: u64,
  pub mod_at: u64,
  pub size: u64,
  pub is_dir: bool,
}

#[test]
fn t1() {
  let file = FileView {
    abs_path: String::from("jack"),
    name: String::from("rose"),
    created_at: 123,
    mod_at: 214,
    size: 52,
    // is_symbol: false,
    is_dir: false,
  };
  let result = serde_json::to_string(&file);
  println!("{:?}", result);
}
