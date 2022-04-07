use std::collections::HashSet;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
#[cfg(windows)]
use std::os::windows::fs::MetadataExt;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use convert_case::{Case, Casing};
use jieba_rs::Jieba;
use pinyin::ToPinyin;
use tantivy::collector::TopDocs;
use tantivy::merge_policy::NoMergePolicy;
use tantivy::query::{BooleanQuery, FuzzyTermQuery, Occur, QueryParser, TermQuery};
use tantivy::schema::*;
use tantivy::{doc, Index, IndexReader, IndexWriter, ReloadPolicy};

use crate::file_view::{FileView, SearchResult};
use crate::utils;
use crate::utils::is_ascii_alphanumeric;
use zhconv::{zhconv, Variant};

lazy_static! {
  pub static ref IDX_STORE: IdxStore = {
    let idx_path = format!("{}{}", utils::data_dir(), "/orangecachedata/idx");
    IdxStore::new(&idx_path)
  };
}

pub struct IdxStore {
  pub writer: Arc<Mutex<IndexWriter>>,
  pub reader: IndexReader,
  pub name_field: Field,
  pub path_field: Field,
  pub query_parser: QueryParser,
  pub is_dir_field: Field,
  pub ext_field: Field,
  pub ext_query_parser: QueryParser,
  pub tokenizer: Jieba,
}

static mut IS_FULL_INDEXING: bool = true;

impl IdxStore {
  pub fn search_tokenize(&self, hans: String) -> String {
    if is_ascii_alphanumeric(hans.as_str()) {
      return self.ascii_tokenize(hans);
    }
    let space = " ";
    let hans = hans
      .replace("-", space)
      .replace("+", space)
      .replace(",", space)
      .replace(".", space)
      .replace("_", space);
    let hans = zhconv(&hans, Variant::ZhHans);
    let words = self.tokenizer.cut(&hans, false);

    let mut token_text: HashSet<String> = vec![].into_iter().collect();

    for word in words {
      token_text.insert(word.to_string());
    }
    token_text.insert(hans.clone());

    token_text.into_iter().collect::<Vec<String>>().join(" ")
  }

  pub fn search_tokenized(&self, hans: String) -> String {
    let s = self.search_tokenize(hans);
    let t = zhconv(&s, Variant::ZhHant);
    format!("{} {}", s, t)
  }

  fn ascii_tokenize(&self, asc: String) -> String {
    let title_lowercase = asc.to_case(Case::Title).to_lowercase();
    let raw_lowercase = asc.to_lowercase();
    // if title_lowercase.eq(&raw_lowercase) {
    //   return title_lowercase;
    // }
    return format!("{} {}", title_lowercase, raw_lowercase);
  }
  pub fn tokenize(&self, hans: String) -> String {
    // return hans;
    if is_ascii_alphanumeric(hans.as_str()) {
      // return hans;
      return self.ascii_tokenize(hans);
    }
    let space = " ";
    let hans = hans.replace("-", space).replace("_", space);
    let hans = zhconv(&hans, Variant::ZhHans);

    let words = self.tokenizer.cut(&hans, false);

    let mut token_text: HashSet<String> = vec![].into_iter().collect();

    for word in words {
      let raw = word;
      let mut first = String::new();
      let mut all = String::new();
      token_text.insert(raw.to_string());
      for pinyin in raw.to_pinyin() {
        if let Some(pinyin) = pinyin {
          first = format!("{}{}", first, pinyin.first_letter());
          all = format!("{}{}", all, pinyin.plain());
        }
      }
      if !first.is_empty() {
        token_text.insert(first);
      }
      if !all.is_empty() {
        token_text.insert(all);
      }
    }
    for pinyin in hans.as_str().to_pinyin() {
      if let Some(full) = pinyin {
        token_text.insert(full.first_letter().to_string());
        token_text.insert(full.plain().to_string());
      }
    }
    token_text.insert(hans.clone());
    token_text.into_iter().collect::<Vec<String>>().join(" ")
  }

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
    let paths = self.search_paths(self.search_tokenize(kw.clone()), limit);
    // if paths.is_empty() {
    //   paths = self.suggest_path(kw, limit);
    // }
    println!("{:?}", paths);
    let file_views = self.parse_file_views(paths);

    file_views
  }

  pub fn search_with_filter(
    &self,
    kw: String,
    limit: usize,
    is_dir_opt: Option<bool>,
    ext_opt: Option<String>,
  ) -> SearchResult {
    let searcher = self.reader.searcher();

    let tokens = self.search_tokenize(kw.clone());
    let kw_query = self.query_parser.parse_query(&tokens).ok().unwrap();
    let mut subqueries = vec![(Occur::Must, kw_query)];

    if let Some(is_dir) = is_dir_opt {
      let is_dir_bytes = IdxStore::is_dir_bytes(is_dir);
      subqueries.push((
        Occur::Must,
        Box::new(TermQuery::new(
          Term::from_field_bytes(self.is_dir_field, is_dir_bytes),
          IndexRecordOption::Basic,
        )),
      ));
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
        .as_bytes()
        .map(|x| std::str::from_utf8(x))
        .unwrap()
        .unwrap();

      paths.push(path.to_string());
    }

    // if paths.is_empty() {
    //   paths = self.suggest_path(kw, limit);
    // }
    let file_views = self.parse_file_views(paths);

    SearchResult {
      file_view: file_views,
      tokenized: self.search_tokenized(kw),
    }
  }

  pub fn suggest(&self, kw: String, limit: usize) -> Vec<FileView> {
    let mut paths = self.search_paths(self.search_tokenize(kw.clone()), limit);
    if paths.is_empty() {
      paths = self.suggest_path(kw, limit);
    }
    let file_views = paths
      .into_iter()
      .map(|x| {
        return FileView {
          abs_path: "".to_string(),
          name: utils::path2name(x),
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
        .as_bytes()
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
      .parse_query(&self.search_tokenize(kw))
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
        .as_bytes()
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
      let path = utils::norm(&path);
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
            name: utils::path2name(path),
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
    let text_field_indexing =
      TextFieldIndexing::default().set_index_option(IndexRecordOption::WithFreqs);
    let text_options = TextOptions::default().set_indexing_options(text_field_indexing);
    let name_field = schema_builder.add_text_field("name", text_options.clone());
    let path_field = schema_builder.add_bytes_field("path", INDEXED | STORED);
    let is_dir_field = schema_builder.add_bytes_field("is_dir_field", INDEXED);
    let ext_field = schema_builder.add_text_field("ext", text_options);
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

    let writer = Arc::new(Mutex::new(
      index.writer_with_num_threads(2, 140_000_000).unwrap(),
    ));
    let writer_bro = writer.clone();
    std::thread::spawn(move || loop {
      let _ = writer_bro.lock().unwrap().commit();
      unsafe {
        if IS_FULL_INDEXING {
          std::thread::sleep(Duration::from_secs(5));
        } else {
          std::thread::sleep(Duration::from_secs(2));
        }
      }
    });

    let reader = index
      .reader_builder()
      .reload_policy(ReloadPolicy::OnCommit)
      .try_into()
      .unwrap();

    let mut query_parser = QueryParser::for_index(&index, vec![name_field]);
    let ext_query_parser = QueryParser::for_index(&index, vec![ext_field]);
    // let mut parent_dirs_query_parser = QueryParser::for_index(&index, vec![parent_dirs_field]);
    query_parser.set_field_boost(name_field, 4.0f32);
    let mut jieba = Jieba::new();
    // it's a feature
    jieba.add_word("陈奕迅", None, None);
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
      tokenizer: jieba, // parent_dirs_query_parser,
    }
  }

  pub fn add(&self, name: String, path: String, is_dir: bool, ext: String) {
    // return;
    unsafe {
      if !IS_FULL_INDEXING {
        self._del(path.clone());
      }
    }
    let mut ext = ext;
    if is_dir {
      ext = "".to_string();
    }
    let is_dir_bytes = IdxStore::is_dir_bytes(is_dir);
    let _ = self.writer.lock().unwrap().add_document(doc!(
        self.name_field => self.tokenize(name),
        self.path_field=>path.as_bytes(),
        self.is_dir_field=>is_dir_bytes,
        self.ext_field=>ext,
        // self.parent_dirs_field=>file_doc.parent_dirs.to_string(),
    ));
  }

  fn is_dir_bytes(is_dir: bool) -> &'static [u8] {
    let is_dir_bytes = if is_dir {
      "1".as_bytes()
    } else {
      "0".as_bytes()
    };
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
  use std::thread::sleep;

  use super::*;

  #[test]
  fn t1() {
    let path = "./tmp";
    let _ = fs::remove_dir_all(path);
    let store = IdxStore::new(path);

    let vec1 = vec![
      "jack rose",
      "JavaHow",
      "patch",
      "patch",
      "patch",
      "data",
      "patch.java",
      "patch.java",
      "DataPatchController.java",
      "patch.java",
      "DataPatchController.java",
      "DataPatchController.java",
      "java",
      "data",
      "data",
    ];

    for x in vec1 {
      store.add(x.to_string(), x.to_string(), false, "".to_string());
    }

    store.commit();
    sleep(Duration::from_secs(1));

    let vec = store.search("datapatchcontroller".to_string(), 10);
    for x in vec {
      println!("{}", x.name);
    }

    // let vec = store.search_paths("data patch".to_string(), 100);
    // for x in vec {
    //   println!("{}", x);
    // }
  }

  #[test]
  fn t2() {
    let idx_path = format!("{}{}", utils::data_dir(), "/orangecachedata/idx");
    let idx_store = Arc::new(IdxStore::new(&idx_path));
    let vec = idx_store.search("data patch controller".to_string(), 10);
    for x in vec {
      println!("{}", x.name);
    }
  }

  #[test]
  fn t5() {
    let idx_path = format!("{}{}", utils::data_dir(), "/orangecachedata/idx");
    let idx_store = Arc::new(IdxStore::new(&idx_path));
    let string = idx_store.tokenize("DataPatchController.java".to_string());
    println!("{}", string);
  }

  #[test]
  fn t6() {
    let hans = zhconv("安全浏览器", Variant::ZhHans);
    println!("{}", hans);
    let hans = zhconv("安全瀏覽器", Variant::ZhHans);
    println!("{}", hans);
  }
}
