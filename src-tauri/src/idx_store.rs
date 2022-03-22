use std::collections::HashSet;

use std::fs;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

use tantivy::collector::TopDocs;
use tantivy::query::{BooleanQuery, FuzzyTermQuery, Occur, Query, QueryParser, TermQuery};
use tantivy::schema::*;

use tantivy::{doc, Index, IndexReader, IndexWriter, ReloadPolicy};

use crate::file_view::FileView;
use crate::pinyin_tokenizer::{search_tokenize, tokenize};
use crate::utils;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::file_doc::FileDoc;

pub struct IdxStore {
  pub writer: Arc<Mutex<IndexWriter>>,
  pub reader: IndexReader,
  pub name_field: Field,
  pub path_field: Field,
  pub query_parser: QueryParser,
  pub is_dir_field: Field,
  pub ext_field: Field,
  pub ext_query_parser: QueryParser,
}

static mut IS_FULL_INDEXING: bool = true;

impl IdxStore {
  pub fn disable_full_indexing(&self) {
    unsafe {
      IS_FULL_INDEXING = false;
    }
  }

  pub fn is_full_indexing(&self) -> bool {
    unsafe {
      return IS_FULL_INDEXING;
    }
  }

  pub fn search(&self, kw: String, limit: usize) -> Vec<FileView> {
    let mut paths = self.search_paths(search_tokenize(kw.clone().to_lowercase()), limit);
    if paths.is_empty() {
      paths = self.suggest_path(kw, limit);
    }
    let file_views = self.parse_file_views(paths);

    file_views
  }

  pub fn search_with_filter(&self, kw: String, limit: usize,  is_dir_opt: Option<bool>,
                            ext_opt: Option<String>,
                            parent_dirs_opt: Option<String>) -> Vec<FileView> {
    let searcher = self.reader.searcher();

    let kw_query = self
        .query_parser
        .parse_query(&search_tokenize(kw.to_lowercase()))
        .ok()
        .unwrap();
    let mut subqueries = vec![
      (Occur::Must, kw_query),
    ];

    if let Some(is_dir) = is_dir_opt {
      let is_dir_bytes = IdxStore::is_dir_bytes(is_dir);
      subqueries.push((Occur::Must, Box::new(TermQuery::new(
        Term::from_field_bytes(self.is_dir_field, is_dir_bytes),
        IndexRecordOption::Basic,
      ))));
    }

    if let Some(ext) = ext_opt {
      let ext_query = self
          .ext_query_parser
          .parse_query(ext.as_str().to_lowercase().as_str())
          .ok()
          .unwrap();

      subqueries.push((Occur::Must, ext_query));
    }



    let q = BooleanQuery::new(subqueries);


    let top_docs = searcher
        .search(&q, &TopDocs::with_limit(limit))
        .ok()
        .unwrap();

    let mut paths = Vec::new();
    for (_score, doc_address) in top_docs {
      let retrieved_doc = searcher.doc(doc_address).ok().unwrap();

      let path = retrieved_doc
          .get_first(self.path_field)
          .unwrap()
          .bytes_value()
          .map(|x| std::str::from_utf8(x))
          .unwrap()
          .unwrap();

      paths.push(path.to_string());
    }

    // if paths.is_empty() {
    //   paths = self.suggest_path(kw, limit);
    // }
    let file_views = self.parse_file_views(paths);

    file_views
  }

  pub fn suggest(&self, kw: String, limit: usize) -> Vec<FileView> {
    let mut paths = self.search_paths(search_tokenize(kw.clone().to_lowercase()), limit);
    if paths.is_empty() {
      paths = self.suggest_path(kw, limit);
    }
    let file_views = paths
      .into_iter()
      .map(|x| {
        return FileView {
          abs_path: "".to_string(),
          name: utils::path2name(utils::norm(&x)).unwrap_or("".to_string()),
          created_at: 0,
          mod_at: 0,
          size: 0,
          is_dir: false,
        };
      })
      .collect::<Vec<FileView>>();
    // let file_views = self.parse_file_views(paths);
    file_views
  }

  fn suggest_path(&self, kw: String, limit: usize) -> Vec<String> {
    let searcher = self.reader.searcher();
    let term = Term::from_field_text(self.name_field, &kw);
    let query = FuzzyTermQuery::new_prefix(term, 1, false);
    let top_docs = searcher
      .search(&query, &TopDocs::with_limit(limit))
      .unwrap();
    let mut paths = Vec::new();
    for (_score, doc_address) in top_docs {
      let retrieved_doc = searcher.doc(doc_address).ok().unwrap();

      let path = retrieved_doc
        .get_first(self.path_field)
        .unwrap()
        .bytes_value()
        .map(|x| std::str::from_utf8(x))
        .unwrap()
        .unwrap();

      paths.push(path.to_string());
    }
    paths
  }

  fn search_paths(&self, kw: String, limit: usize) -> Vec<String> {
    let searcher = self.reader.searcher();

    let query = self
      .query_parser
      .parse_query(&search_tokenize(kw))
      .ok()
      .unwrap();

    let top_docs = searcher
      .search(&query, &TopDocs::with_limit(limit))
      .ok()
      .unwrap();

    let mut paths = Vec::new();
    for (_score, doc_address) in top_docs {
      let retrieved_doc = searcher.doc(doc_address).ok().unwrap();

      let path = retrieved_doc
        .get_first(self.path_field)
        .unwrap()
        .bytes_value()
        .map(|x| std::str::from_utf8(x))
        .unwrap()
        .unwrap();

      paths.push(path.to_string());
    }
    paths
  }

  fn parse_file_views(&self, paths: Vec<String>) -> Vec<FileView> {
    let mut file_views = Vec::new();

    let mut uniques: HashSet<String> = HashSet::new();

    for path in paths {
      match fs::metadata(path.clone()) {
        Ok(meta) => {
          if !uniques.contains(&path) {
            uniques.insert(path.clone());
          } else {
            continue;
          }
          #[cfg(windows)]
          let size = meta.file_size();
          #[cfg(unix)]
          let size = meta.size();

          file_views.push(FileView {
            abs_path: utils::norm(&path),
            name: utils::path2name(utils::norm(&path)).unwrap_or("".to_string()),
            created_at: meta
              .created()
              .unwrap_or(SystemTime::now())
              .duration_since(UNIX_EPOCH)
              .unwrap()
              .as_secs(),
            mod_at: meta
              .modified()
              .unwrap_or(SystemTime::now())
              .duration_since(UNIX_EPOCH)
              .unwrap()
              .as_secs(),
            size: size,
            is_dir: meta.is_dir(),
          });
        }
        Err(_) => {}
      }
    }

    file_views
  }

  pub fn _del(&self, abs_path: String) {
    let term = Term::from_field_bytes(self.path_field, abs_path.as_bytes());
    self.writer.lock().unwrap().delete_term(term);
  }

  pub fn new(path: &str) -> IdxStore {
    let index_path = std::path::Path::new(path);
    let mut schema_builder = Schema::builder();
    let name_field = schema_builder.add_text_field("name", TEXT | STORED);
    let path_field = schema_builder.add_bytes_field("path", INDEXED );
    let is_dir_field = schema_builder.add_bytes_field("is_dir_field", INDEXED );
    let ext_field = schema_builder.add_text_field("ext", TEXT );
    // let parent_dirs_field = schema_builder.add_text_field("parent_dirs", TEXT );
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

    let writer = Arc::new(Mutex::new(index.writer(50_000_000).unwrap()));
    let writer_bro = writer.clone();
    std::thread::spawn(move || loop {
      let _ = writer_bro.lock().unwrap().commit();
      unsafe {
        if IS_FULL_INDEXING {
          std::thread::sleep(Duration::from_secs(5));
        } else {
          std::thread::sleep(Duration::from_secs(1));
        }
      }
    });

    let reader = index
      .reader_builder()
      .reload_policy(ReloadPolicy::OnCommit)
      .try_into()
      .unwrap();

    let mut query_parser = QueryParser::for_index(&index, vec![name_field]);
    let mut ext_query_parser = QueryParser::for_index(&index, vec![ext_field]);
    // let mut parent_dirs_query_parser = QueryParser::for_index(&index, vec![parent_dirs_field]);
    query_parser.set_field_boost(name_field, 4.0f32);

    IdxStore {
      writer,
      reader,
      name_field,
      path_field,
      // parent_dirs_field,
      ext_field,
      is_dir_field,
      query_parser,
      ext_query_parser,
      // parent_dirs_query_parser,
    }
  }

  pub fn add(&self, file_doc:FileDoc) {
    unsafe {
      if !IS_FULL_INDEXING {
        self._del(file_doc.path.to_string());
      }
    }
    let is_dir = file_doc.is_dir;
    let is_dir_bytes = IdxStore::is_dir_bytes(is_dir);
    self.writer.lock().unwrap().add_document(doc!(
        self.name_field => tokenize(file_doc.name.to_string()),
        self.path_field=>file_doc.path.as_bytes(),
        self.is_dir_field=>is_dir_bytes,
        self.ext_field=>file_doc.ext,
        // self.parent_dirs_field=>file_doc.parent_dirs.to_string(),
    ));
  }

  fn is_dir_bytes(is_dir: bool) -> &'static [u8] {
    let is_dir_bytes = if is_dir { "1".as_bytes() } else { "0".as_bytes() };
    is_dir_bytes
  }

  pub fn commit(&self) {
    let _ = self.writer.lock().unwrap().commit();
  }

  pub fn num_docs(&self) -> u64 {
    self.reader.searcher().num_docs()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn t1() {
    let mut store = IdxStore::new("./tmp");
    // store.add("jack", "rose");
    // store.add("jack", "rose大萨达");
    store.commit();
    let vec = store.search("jack".to_string(), 12);
    println!("{}", store.num_docs());
  }

  #[test]
  fn t2() {
    let idx_path = format!("{}{}", utils::data_dir(), "/orangecachedata/idx");
    let idx_store = Arc::new(IdxStore::new(&idx_path));
    let vec = idx_store.search_with_filter("SearchBox".to_string(), 100,
                                           None, None, Some("/Users/jeff/IdeaProjects/orange2".to_string()));
    for x in vec {
      println!("{}", x.name);
    }
  }


}
