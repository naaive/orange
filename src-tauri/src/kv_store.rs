use kv::*;

#[derive(Clone)]
pub struct KvStore<'a> {
  bucket: Bucket<'a, String, String>,
}

impl KvStore<'_> {
  pub fn new<'a>(path: &str) -> KvStore<'a> {
    let cfg = Config::new(path);
    let store = Store::new(cfg).unwrap();
    let result = store.bucket::<String, String>(Some("aaa"));
    let bucket = result.unwrap();
    KvStore { bucket }
  }

  pub fn _del(&self, k: String) {
    let _ = self.bucket.remove(k).ok();
  }

  pub fn clear(&self) {
    let _ = self.bucket.clear();
  }

  pub fn put_str(&self, k: String, v: String) {
    self.bucket.set(k, v).unwrap();
    self.bucket.flush().ok();
  }

  pub fn get_str(&self, k: String) -> Option<String> {
    self.bucket.get(k).ok().unwrap()
  }
}

#[test]
fn kv_test() {}
