use std::path::Path;
use crate::file_index::FileIndex;
use crate::file_kv::FileKv;
use crate::file_view::FileView;
use crate::kv_store::KvStore;
use crate::IndexStore;

pub struct UnitedStore<'a> {
    kv: KvStore<'a>,
    idx: IndexStore,
}

impl UnitedStore<'_> {
    pub fn new<'a>() -> UnitedStore<'a> {
        let kv = KvStore::new("./cachedata/kv");
        let idx = IndexStore::new();
        UnitedStore { kv, idx }
    }
    pub fn save(&mut self, file: FileView) {
        let abs_path = file.abs_path;
        let abs_path_clone1 = abs_path.clone();
        let abs_path_clone2 = abs_path.clone();
        let name = file.name;
        let created_at = file.created_at;
        let size = file.size;
        let mod_at = file.mod_at;
        let is_dir = file.is_dir;
        // let is_symbol = file.is_symbol;
        let opt = self.kv.get(abs_path.clone());

        match opt {
            None => {
                let kv = FileKv {
                    abs_path,
                    created_at,
                    mod_at,
                    size,
                    is_dir
                    // is_symbol,
                };
                self.kv.put(abs_path_clone1, kv);
                self.idx.add_doc(FileIndex {
                    abs_path: abs_path_clone2,
                    name,
                })
            }
            Some(_) => {
                let kv = FileKv {
                    abs_path,
                    created_at,
                    mod_at,
                    size,
                    is_dir
                    // is_symbol,
                };
                self.kv.put(abs_path_clone1, kv);
            }
        }
    }

    pub fn del(&self, path: &str) {
        self.kv.del(String::from(path));
        self.idx.del(String::from(path));
    }

    pub fn search(&self, kw: &str, limit: usize) -> Vec<FileView> {
        let mut file_views = Vec::new();
        let arr = self.idx.search(String::from(kw), limit);
        for x in arr {
            let file_opt = self.kv.get(x);
            match file_opt {
                None => {}
                Some(fkv) => {

                    file_views.push(FileView {
                        abs_path: fkv.abs_path.clone(),
                        name: Path::new(fkv.abs_path.as_str()).file_name().unwrap().to_str().unwrap().to_string(),
                        created_at: fkv.created_at,
                        mod_at: fkv.mod_at,
                        size: fkv.size,
                        // is_symbol: fkv.is_symbol
                        is_dir: fkv.is_dir
                    })
                }
            }
        }
        file_views
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let mut store = UnitedStore::new();
        let view = FileView {
            abs_path: "hello".to_string(),
            name: "world".to_string(),
            created_at: 0,
            mod_at: 0,
            size: 88,
            // is_symbol: false
            is_dir: false
        };
        store.save(view);
        let option = store.kv.get("hello".to_string());
        println!("{:?}", option);
    }

    #[test]
    fn t2() {
        let store = UnitedStore::new();
        let x = store.search("usr", 100);
        println!("{:?}", x);
    }
}
