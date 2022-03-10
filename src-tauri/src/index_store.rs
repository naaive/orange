use std::cell::RefCell;
use std::collections::HashSet;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::Field;
use tantivy::schema::Schema;
use tantivy::schema::*;
use tantivy::{doc, Index, IndexReader, IndexWriter, ReloadPolicy};

use crate::file_index::FileIndex;
use crate::file_view::FileView;
use crate::pinyin_tokenizer::tokenize;
use crate::utils;

pub struct IndexStore {
  index: Index,
  index_writer:  Arc<RwLock<IndexWriter>>,
  index_reader: IndexReader,
  name_field: Field,
  abs_path_field: Field,
}

impl IndexStore {
  pub fn new(path: &str) -> IndexStore {
    let index_path = Path::new(path);
    let mut schema_builder = Schema::builder();
    let name_field = schema_builder.add_text_field("name", TEXT | STORED);
    let abs_path_field = schema_builder.add_text_field("path", STORED);
    let schema = schema_builder.build();

    let index;
    if index_path.exists() {
      index = Index::open_in_dir(&index_path).ok().unwrap();
    } else {
      fs::create_dir(index_path).ok();
      index = Index::create_in_dir(&index_path, schema.clone())
        .ok()
        .unwrap();
    }

    // let mut index_writer = index.writer(50_000_000).unwrap();

    let index_writer: Arc<RwLock<IndexWriter>> =
        Arc::new(RwLock::new(index.writer(50_000_000).ok().unwrap()));


    // let iw_clone = index_writer.clone();
    // std::thread::spawn(move || loop {
    //   iw_clone.write().unwrap().commit().ok();
    //   std::thread::sleep(Duration::from_secs(1))
    // });



    let index_reader = index
      .reader_builder()
      .reload_policy(ReloadPolicy::Manual)
      .try_into()
      .ok()
      .unwrap();
    IndexStore {
      index,
      index_writer,
      index_reader,
      name_field,
      abs_path_field,
    }
  }


  pub fn add_doc(&self, file_index: FileIndex) {
    let tokens = tokenize(file_index.name);
    self.index_writer.write().unwrap().add_document(doc!(
        self.name_field => tokens,
        self.abs_path_field =>file_index.abs_path
    ));
  }

  pub fn search(&self, kw: String, limit: usize) -> Vec<FileView> {
    let searcher = self.index_reader.searcher();
    let mut query_parser = QueryParser::for_index(&self.index, vec![self.name_field]);
    // query_parser.set_field_boost(self.abs_path_field, 1.0f32);
    query_parser.set_field_boost(self.name_field, 4.0f32);

    let query = query_parser
      .parse_query(kw.as_str().to_lowercase().as_str())
      .ok()
      .unwrap();

    let top_docs = searcher
      .search(&query, &TopDocs::with_limit(limit))
      .ok()
      .unwrap();

    let mut res = Vec::new();
    for (_score, doc_address) in top_docs {
      let retrieved_doc = searcher.doc(doc_address).ok().unwrap();

      let x = retrieved_doc
        .get_first(self.abs_path_field)
        .unwrap()
        .text()
        .unwrap();

      res.push(x.to_string());
    }
    let mut file_views = Vec::new();

    let set: HashSet<String> = HashSet::from_iter(res);
    for x in set {
      match fs::metadata(x.clone()) {
        Ok(meta) => {
          #[cfg(windows)]
          let size = meta.file_size();
          #[cfg(unix)]
          let size = meta.size();

          file_views.push(FileView {
            abs_path: x.clone(),
            name: utils::path2name(x.as_str()).unwrap_or("").to_string(),
            created_at: utils::parse_ts(meta.created().unwrap_or(SystemTime::now())),
            mod_at: utils::parse_ts(meta.modified().unwrap_or(SystemTime::now())),
            size: size,
            // is_symbol: fkv.is_symbol
            is_dir: meta.is_dir(),
          })
        }
        Err(_) => {}
      }
    }

    file_views
  }

  pub fn commit(&self) {
    self.index_writer.write().unwrap().commit();

  }

  pub fn del(&self, abs_path: String) {
    let term = Term::from_field_text(self.abs_path_field, &abs_path);
    self.index_writer.write().unwrap().delete_term(term);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::data_dir;
  use std::time::SystemTime;
  use walkdir::WalkDir;

  #[test]
  fn t2() {
    let mut store = IndexStore::new(&format!("{}{}", data_dir(), "/orangecachedata/idx"));
    store.search("jeff".to_string(), 100);
  }
  #[test]
  fn t1() {
    let string = format!(
      "{}{}",
      "/Users/jeff/IdeaProjects/orange2/src-tauri/target/", "/index"
    );
    let mut store = IndexStore::new(&string);

    let root = "/Users/jeff/CLionProjects/orange2";

    println!("start travel");
    let mut cnt = 0;
    let start = SystemTime::now();

    for entry in WalkDir::new(root) {
      cnt += 1;
      let entry1 = entry.unwrap();
      let buf = entry1.path();
      let x = buf.to_str().unwrap();
      let x1 = entry1.file_name().to_str().unwrap();
      store.add_doc(FileIndex {
        name: x1.to_string(),
        abs_path: x,
      });
    }

    let end = SystemTime::now();

    println!(
      "cost {} ms, total {} files",
      end.duration_since(start).unwrap().as_millis(),
      cnt
    );
  }
}
