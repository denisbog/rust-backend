use std::path::PathBuf;

use tantivy::{
    schema::{Schema, FAST, INDEXED, TEXT},
    Index,
};

pub fn get_schema() -> tantivy::schema::Schema {
    let mut schema_builder = Schema::builder();
    schema_builder.add_u64_field("id", INDEXED | FAST);
    schema_builder.add_text_field("title", TEXT);
    schema_builder.add_text_field("description", TEXT);
    schema_builder.add_text_field("category", TEXT);
    schema_builder.add_text_field("subcategory", TEXT);
    schema_builder.build()
}
pub fn get_index() -> tantivy::Index {
    // Indexing documents
    let index_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("index_data");

    if !index_dir.exists() {
        std::fs::create_dir_all(&index_dir).unwrap();
    }

    let dir = tantivy::directory::MmapDirectory::open(index_dir).unwrap();
    Index::open_or_create(dir, get_schema()).unwrap()
}
