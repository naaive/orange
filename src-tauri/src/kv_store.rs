use crate::file_kv::FileKv;
use kv::*;

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

    pub fn put(&mut self, k: String, v: FileKv) {
        let string = serde_json::to_string(&v).ok().unwrap();
        self.bucket.set(k, string).unwrap();
    }

    pub fn del(&self, k: String) {
        self.bucket.remove(k);
    }


    pub fn put_str(&mut self, k: String, v: String) {
        let string = serde_json::to_string(&v).ok().unwrap();
        self.bucket.set(k, string).unwrap();
        self.bucket.flush();
    }

    pub fn get_str(&self, k: String) -> Option<String> {
        self.bucket
            .get(k)
            .ok()
            .unwrap()
            .map(|x| serde_json::from_str(&x).ok().unwrap())

    }
    pub fn get(&self, k: String) -> Option<FileKv> {
        self.bucket
            .get(k)
            .ok()
            .unwrap()
            .map(|x| serde_json::from_str(&x).ok().unwrap())
    }
}

#[test]
fn kv_test() {
    let mut b = KvStore::new("./test/example1");
    b.put(
        String::from("xx"),
        FileKv {
            abs_path: "jack".to_string(),
            created_at: 0,
            mod_at: 0,
            size: 21,
            // is_symbol: true
            is_dir: false
        },
    );
    let option = b.get(String::from("xx"));
    println!("{:?}", option);
}
