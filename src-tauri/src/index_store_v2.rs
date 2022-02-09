use std::fs;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, IndexReader, IndexWriter, ReloadPolicy};

use crate::file_index::FileIndex;
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::time::Duration;

pub struct IndexStore {
    index_writer: Arc<RwLock<IndexWriter>>,
    // index_reader: IndexReader,
    // searcher: LeasedItem<Searcher>,
    index: Index,
    schema: Schema,
    index_reader: IndexReader,
    pub name_field: Field,
    pub abs_path_filed: Field,
}

impl IndexStore {
    pub fn new() -> IndexStore {
        let index_path = Path::new("./index");

        let mut schema_builder = Schema::builder();

        let abs_path = "abs_path";
        let name = "name";

        schema_builder.add_text_field(abs_path, TEXT | STORED);
        schema_builder.add_text_field(name, TEXT);
        let schema = schema_builder.build();

        let abs_path_filed = schema.get_field(abs_path).unwrap();
        let name_field = schema.get_field(name).unwrap();
        let index;
        if index_path.exists() {
            index = Index::open_in_dir(&index_path).ok().unwrap();
        } else {
            fs::create_dir(index_path);
            index = Index::create_in_dir(&index_path, schema.clone())
                .ok()
                .unwrap();
        }
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
            iw_clone.write().unwrap().commit();
            std::thread::sleep(Duration::from_secs(1))
        });

        return IndexStore {
            abs_path_filed,
            name_field,
            index_writer,
            index_reader,
            index,
            schema,
        };
    }

    pub fn search(&self, kw: String, limit: usize) -> Vec<String> {
        let searcher = self.index_reader.searcher();

        let query_parser =
            QueryParser::for_index(&self.index, vec![self.abs_path_filed, self.name_field]);

        let query = query_parser.parse_query(kw.as_str()).ok().unwrap();

        let top_docs = searcher
            .search(&query, &TopDocs::with_limit(limit))
            .ok()
            .unwrap();
        let mut vec1 = Vec::new();
        for (_score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc(doc_address).ok().unwrap();

            let x = retrieved_doc
                .get_first(self.abs_path_filed)
                .unwrap()
                .text()
                .unwrap();

            vec1.push(x.to_string());
        }
        vec1
    }
    pub fn del(&self, abs_path: String) {
        let term = Term::from_field_text(self.abs_path_filed, &abs_path);
        self.index_writer.write().unwrap().delete_term(term);
    }

    pub fn add_doc(&mut self, file_index: FileIndex) {
        let json = serde_json::to_string(&file_index).unwrap();
        let doc = self.schema.parse_document(&json).unwrap();
        self.index_writer.write().unwrap().add_document(doc);
    }
    pub fn commit(&mut self) {
        self.index_writer.write().unwrap().commit();
    }
}

#[test]
fn t1() {
    let mut storev2 = IndexStore::new();

    storev2.add_doc(FileIndex {
        abs_path: "lorem".to_string(),
        name: "ooo".to_string(),
    });
    storev2.add_doc(FileIndex {
        abs_path: "io".to_string(),
        name: "jason".to_string(),
    });
    storev2.add_doc(FileIndex {
        abs_path: "json".to_string(),
        name: "micheal".to_string(),
    });
    storev2.add_doc(FileIndex {
        abs_path: "jeff".to_string(),
        name: "old".to_string(),
    });
    std::thread::sleep(Duration::from_secs(5));
    let vec = storev2.search(String::from("jeff"), 100);
    println!("{:?}", vec);
}
