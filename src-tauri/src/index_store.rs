use std::fs;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::Schema;
use tantivy::schema::*;

use tantivy::{Index, IndexReader, IndexWriter, ReloadPolicy};

use crate::file_index::FileIndex;
use sha256::digest;
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::time::Duration;

#[derive(Clone)]
pub struct IndexStore {
  index_writer: Arc<RwLock<IndexWriter>>,
  // index_reader: IndexReader,
  // searcher: LeasedItem<Searcher>,
  index: Index,
  schema: Schema,
  index_reader: IndexReader,
  pub name_field: Field,
  pub id_field: Field,
  pub abs_path_field: Field,
}

impl IndexStore {
  pub fn new() -> IndexStore {
    let index_path = Path::new("./orangecachedata/index");

    let mut schema_builder = Schema::builder();

    let id = "id";
    let abs_path = "abs_path";
    let name = "name";

    let text_field_indexing = TextFieldIndexing::default()
      .set_tokenizer("jieba")
      .set_index_option(IndexRecordOption::WithFreqsAndPositions);
    let text_store_options = TextOptions::default()
      .set_indexing_options(text_field_indexing.clone())
      .set_stored();
    let text_options = TextOptions::default().set_indexing_options(text_field_indexing);

    schema_builder.add_text_field(id, TextOptions::default());
    schema_builder.add_text_field(abs_path, text_store_options);
    schema_builder.add_text_field(name, text_options.clone());
    let schema = schema_builder.build();

    let id_field = schema.get_field(id).unwrap();
    let abs_path_field = schema.get_field(abs_path).unwrap();
    let name_field = schema.get_field(name).unwrap();
    let index;
    if index_path.exists() {
      index = Index::open_in_dir(&index_path).ok().unwrap();
    } else {
      fs::create_dir(index_path).ok();
      index = Index::create_in_dir(&index_path, schema.clone())
        .ok()
        .unwrap();
    }
    let tokenizer = tantivy_jieba::JiebaTokenizer {};
    index.tokenizers().register("jieba", tokenizer);
    // let index_writer = index.writer(50_000_000).ok().unwrap();
    let index_writer: Arc<RwLock<IndexWriter>> =
      Arc::new(RwLock::new(index.writer(50_000_000).ok().unwrap()));

    let index_reader = index
      .reader_builder()
      .reload_policy(ReloadPolicy::OnCommit)
      .try_into()
      .ok()
      .unwrap();

    let iw_clone = index_writer.clone();
    std::thread::spawn(move || loop {
      iw_clone.write().unwrap().commit().ok();
      std::thread::sleep(Duration::from_secs(1))
    });

    return IndexStore {
      abs_path_field,
      name_field,
      id_field,
      index_writer,
      index_reader,
      index,
      schema,
    };
  }

  pub fn search(&self, kw: String, limit: usize) -> Vec<String> {
    let searcher = self.index_reader.searcher();
    let mut query_parser =
      QueryParser::for_index(&self.index, vec![self.abs_path_field, self.name_field]);
    query_parser.set_field_boost(self.abs_path_field, 1.0f32);
    query_parser.set_field_boost(self.name_field, 4.0f32);

    let query = query_parser
      .parse_query(kw.as_str().to_lowercase().as_str())
      .ok()
      .unwrap();

    let top_docs = searcher
      .search(&query, &TopDocs::with_limit(limit))
      .ok()
      .unwrap();

    let mut vec1 = Vec::new();
    for (_score, doc_address) in top_docs {
      let retrieved_doc = searcher.doc(doc_address).ok().unwrap();

      let x = retrieved_doc
        .get_first(self.abs_path_field)
        .unwrap()
        .text()
        .unwrap();

      vec1.push(x.to_string());
    }

    vec1
  }
  pub fn del(&self, abs_path: String) {
    let term = Term::from_field_text(self.id_field, &digest(&abs_path));
    self.index_writer.write().unwrap().delete_term(term);
  }

  pub fn add_doc(&mut self, file_index: FileIndex) {
    let json = serde_json::to_string(&file_index).unwrap();
    let doc = self.schema.parse_document(&json).unwrap();
    self.index_writer.write().unwrap().add_document(doc);
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  use log::{debug, error};
  use std::time::SystemTime;
  use tantivy::tokenizer::Tokenizer;

  #[test]
  fn t3() {
    env_logger::init();

    let storev2 = IndexStore::new();

    // std::thread::sleep(Duration::from_secs(1));
    let time = SystemTime::now();
    storev2.search(String::from("思维"), 100);
    let time2 = SystemTime::now();
    println!(
      "elapsed {} ms",
      time2.duration_since(time).unwrap().as_millis()
    );

    let time = SystemTime::now();
    let vec = storev2.search(String::from("抽象"), 100);
    let time2 = SystemTime::now();
    println!(
      "elapsed {} ms",
      time2.duration_since(time).unwrap().as_millis()
    );
    println!("{:?}", vec);
  }

  #[test]
  fn t6() {
    env_logger::init();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");
  }
  #[test]
  fn t7() {
    let x = "C:\\Users\\Administrator\\Desktop\\新建文件夹\\jason - 副本.txt";
    let x1 = "C:\\Users\\Administrator\\Desktop\\新建文件夹\\jason - 副本 (2).txt";

    let string = digest(x);
    println!("{}", string);
    let string1 = digest(x1);
    println!("{}", string1);
    let tokenizer = tantivy_jieba::JiebaTokenizer {};
    let mut token_stream = tokenizer.token_stream(string.as_str());
    println!("{}", token_stream.next().unwrap().text);

    let tokenizer = tantivy_jieba::JiebaTokenizer {};
    let mut token_stream = tokenizer.token_stream(string1.as_str());
    println!("{}", token_stream.next().unwrap().text);
  }
}
