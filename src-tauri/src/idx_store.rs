use std::collections::HashSet;

use std::fs;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

use tantivy::collector::{TopDocs};
use tantivy::query::{FuzzyTermQuery, QueryParser};
use tantivy::schema::*;

use tantivy::{doc, Index, IndexReader, IndexWriter, ReloadPolicy};

use crate::file_view::FileView;
use crate::pinyin_tokenizer::tokenize;
use crate::utils;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct IdxStore {
  pub writer: Arc<Mutex<IndexWriter>>,
  pub reader: IndexReader,
  pub name_field: Field,
  pub path_field: Field,
  pub query_parser: QueryParser,
}

static mut IS_FULL_INDEXING: bool = true;

impl IdxStore {
  pub fn disable_full_indexing(&self) {
    unsafe {
      IS_FULL_INDEXING = false;
    }
  }

  pub fn search(&self, kw: String, limit: usize) -> Vec<FileView> {
    let paths = self.search_paths(kw, limit);

    let file_views = self.parse_file_views(paths);

    file_views
  }

  pub fn suggest(&self, kw: String, limit: usize) -> Vec<FileView> {
    let searcher = self.reader.searcher();
    let term = Term::from_field_text(self.name_field, &kw);
    let query = FuzzyTermQuery::new(term, 1, true);
    let top_docs = searcher
      .search(&query, &TopDocs::with_limit(limit))
      .unwrap();
    let mut paths = Vec::new();
    for (_score, doc_address) in top_docs {
      let retrieved_doc = searcher.doc(doc_address).ok().unwrap();

      let path = retrieved_doc
        .get_first(self.path_field)
        .unwrap()
        .text()
        .unwrap();

      paths.push(path.to_string());
    }
    let file_views = self.parse_file_views(paths);
    file_views
  }

  fn search_paths(&self, kw: String, limit: usize) -> Vec<String> {
    let searcher = self.reader.searcher();

    let query = self
      .query_parser
      .parse_query(kw.as_str().to_lowercase().as_str())
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
        .text()
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

  pub fn del(&self, abs_path: String) {
    let term = Term::from_field_text(self.path_field, &abs_path);
    self.writer.lock().unwrap().delete_term(term);
  }

  pub fn new(path: &str) -> IdxStore {
    let index_path = std::path::Path::new(path);
    let mut schema_builder = Schema::builder();
    let name_field = schema_builder.add_text_field("name", TEXT | STORED);
    let path_field = schema_builder.add_text_field("path", STORED);
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
      writer_bro.lock().unwrap().commit();
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
    query_parser.set_field_boost(name_field, 4.0f32);

    IdxStore {
      writer,
      reader,
      name_field,
      path_field,
      query_parser,
    }
  }

  pub fn add(&self, name: &str, path: &str) {
    self.writer.lock().unwrap().add_document(doc!(
        self.name_field => tokenize(name.to_string()),
        self.path_field=>path
    ));
  }

  pub fn commit(&self) {
    let _ = self.writer.lock().unwrap().commit();
  }
}
