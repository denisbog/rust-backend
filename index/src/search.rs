use std::sync::Arc;
use std::time::Duration;
use std::u64;

use crate::setup::get_index;
use common::DbItem;
use tantivy::collector::{Count, TopDocs};
use tantivy::query::QueryParser;
use tantivy::schema::Term;
use tantivy::{IndexWriter, ReloadPolicy};

pub struct SearchEngine {
    query_parser: QueryParser,
    reader: tantivy::IndexReader,
    index: tantivy::Index,
    index_writer: Arc<tokio::sync::RwLock<IndexWriter>>,
    dirty_operations: Arc<tokio::sync::Mutex<i32>>,
}

pub struct SearchResults {
    pub items: Vec<u64>,
    pub count: i32,
}

// macro_rules! add_text {
//     ( $document:ident ,$schema:ident => $item :ident , $attr:literal ) => {
//         if let Some(value) = &$item.$attr {
//             $document.add_text($schema.get_field("$attr").unwrap(), value);
//         }
//     };
// }
//
//

impl Default for SearchEngine {
    fn default() -> Self {
        let index = get_index();

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()
            .unwrap();

        let mut query_parser = QueryParser::for_index(
            &index,
            vec![
                index.schema().get_field("title").unwrap(),
                index.schema().get_field("description").unwrap(),
            ],
        );

        query_parser.set_field_fuzzy(index.schema().get_field("title").unwrap(), true, 1, false);
        query_parser.set_field_fuzzy(
            index.schema().get_field("description").unwrap(),
            true,
            1,
            false,
        );

        let index_writer = index.writer(20_000_000).unwrap();

        SearchEngine {
            index,
            query_parser,
            reader,
            index_writer: Arc::new(tokio::sync::RwLock::new(index_writer)),
            dirty_operations: Arc::new(tokio::sync::Mutex::new(0)),
        }
    }
}
impl SearchEngine {
    pub async fn start(search_engine: Arc<SearchEngine>) {
        tokio::spawn({
            async move {
                let mut interval = tokio::time::interval(Duration::from_secs(30));
                loop {
                    interval.tick().await;
                    search_engine.monitor_commit().await;
                }
            }
        });
    }

    pub fn search(&self, search: &str, results: usize, offset: usize) -> SearchResults {
        let query = self.query_parser.parse_query(search).unwrap();

        let searcher = self.reader.searcher();
        let count = searcher.search(&query, &Count).unwrap();

        let query_results = searcher
            .search(&query, &TopDocs::with_limit(results).and_offset(offset))
            .unwrap();
        let items = query_results
            .iter()
            .map(|(_score, doc_address)| {
                let out = searcher
                    .segment_reader(doc_address.segment_ord)
                    .fast_fields()
                    .u64("id")
                    .unwrap();
                out.first(doc_address.doc_id).unwrap()
            })
            .collect::<Vec<u64>>();
        SearchResults {
            items,
            count: i32::try_from(count).unwrap(),
        }
    }
    pub async fn index(&self, item: &DbItem) {
        let id = item.id.unwrap();
        self.delete(id).await;

        let schema = self.index.schema();
        let mut document = tantivy::Document::new();

        document.add_u64(schema.get_field("id").unwrap(), id);
        // add_text!(document, schema => item, title);

        if let Some(title) = &item.title {
            document.add_text(schema.get_field("title").unwrap(), title);
        }
        if let Some(description) = &item.description {
            document.add_text(schema.get_field("description").unwrap(), description);
        }
        if let Some(category) = &item.category {
            document.add_text(schema.get_field("category").unwrap(), category);
        }
        if let Some(subcategory) = &item.subcategory {
            document.add_text(schema.get_field("subcategory").unwrap(), subcategory);
        }

        let index_writer = self.index_writer.read().await;
        index_writer.add_document(document).unwrap();
        self.dirty().await;
    }

    pub async fn delete(&self, id: u64) {
        let schema = self.index.schema();
        let term = Term::from_field_u64(schema.get_field("id").unwrap(), id);

        let index_writer = self.index_writer.read().await;
        index_writer.delete_term(term);
        self.dirty().await;
    }

    async fn dirty(&self) {
        *self.dirty_operations.lock().await += 1;
    }

    pub async fn commit(&self) {
        self.index_writer.write().await.commit().unwrap();
        tracing::info!("commit");
    }

    pub async fn reset_index(&self) {
        let mut writer = self.index_writer.write().await;
        writer.delete_all_documents().unwrap();
        writer.commit().unwrap();
    }

    async fn monitor_commit(&self) {
        if *self.dirty_operations.lock().await > 0 {
            self.index_writer.write().await.commit().unwrap();
            tracing::info!("commit");
            *self.dirty_operations.lock().await = 0;
            //  } else {
            //     info!("no need for commit");
        }
    }
}
