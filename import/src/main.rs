use std::{collections::HashMap, io::BufRead, path::PathBuf, sync::Arc};

use common::{DbItem, Point};
use index::search::SearchEngine;
use serde_derive::Deserialize;
use serde_dynamo::AttributeValue;
use sqlx::MySqlPool;

#[derive(Deserialize)]
struct Place {
    lat: f64,
    lng: f64,
}

#[derive(Deserialize)]
struct Item {
    // id: String,
    title: Option<String>,
    description: Option<String>,
    category: Option<String>,
    subcategory: Option<String>,
    // user: Option<String>,
    address: Option<String>,
    #[serde(rename = "priceType")]
    price_type: Option<String>,
    // created: Option<String>,
    place: Option<Place>,
    images: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct Wrap {
    #[serde(rename = "Item")]
    item: HashMap<String, AttributeValue>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let data_file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("backup/all.json");

    tracing::info!("import from file: {:?}", data_file);
    let items: Vec<Item> = std::io::BufReader::new(std::fs::File::open(data_file).unwrap())
        .lines()
        .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
        .map(|item: Wrap| serde_dynamo::from_item(serde_dynamo::Item::from(item.item)).unwrap())
        .collect();
    let search_engine = Arc::new(SearchEngine::default());

    search_engine.reset_index().await;
    SearchEngine::start(search_engine.clone()).await;

    let db_connection_str = std::env::var("DATABASE_URL").unwrap();
    let pool = MySqlPool::connect(&db_connection_str).await.unwrap();

    sqlx::query!("delete from items")
        .execute(&pool)
        .await
        .unwrap();

    futures::future::join_all(items.into_iter().map(|item| async {
        store(&pool, &search_engine, item).await;
    }))
    .await;
    search_engine.commit().await;
}

async fn store(pool: &MySqlPool, search_engine: &SearchEngine, item: Item) {
    if let (Some(title), Some(place), Some(category), Some(subcategory)) =
        (&item.title, &item.place, &item.category, &item.subcategory)
    {
        let id = sqlx::query_scalar!(
            r#"
    INSERT INTO items(
        title,
        description,
        price_type,
        price,
        location,
        place_description,
        category,
        subcategory,
        image,
        user,
        status
        )
    VALUES (?, ?, ?, ?, Point(?,?), ?, ?, ?, ?, ?, ?)
            "#,
            title,
            item.description,
            item.price_type,
            100,
            place.lat,
            place.lng,
            item.address,
            category,
            subcategory,
            "default.jpeg",
            "generated",
            "all",
        )
        .execute(pool)
        .await
        .unwrap()
        .last_insert_id();

        copy_image(id, &item).await;

        let db_item = DbItem {
            id: Some(id),
            title: item.title,
            description: item.description,
            category: item.category,
            subcategory: item.subcategory,
            image: Some("default.jpeg".to_string()),
            created: None,
            location: Some(Point {
                lat: item.place.as_ref().unwrap().lat,
                lng: item.place.as_ref().unwrap().lng,
            }),
            price: Some(100f64),
            place_description: item.address,
            price_type: item.price_type,
            reserved: None,
            status: None,
            updated: None,
            user: Some("genered".into()),
        };

        search_engine.index(&db_item).await;
    };
}

async fn copy_image(id: u64, item: &Item) -> String {
    let content_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../server/content")
        .join(id.to_string());

    if tokio::fs::create_dir_all(&content_folder).await.is_err() {
        println!("issue while creating destination folder, skip creation, continue with copy");
    }

    let image_name = item.images.as_ref().unwrap().iter().next().unwrap();

    let image_name = image_name.rsplit_once('/').unwrap().1;

    let noimage_src = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("backup/images")
        .join("noimage.jpg");
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("backup/images")
        .join(image_name);
    let dest = content_folder.join("default.jpeg");
    if tokio::fs::copy(&src, &dest).await.is_err() {
        println!("error while copying {:?} to {:?}", src, dest);
        let image_name = "noimage.jpeg";
        tokio::fs::copy(&noimage_src, &dest).await.unwrap();
        image_name.to_string()
    } else {
        image_name.to_string()
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    #[test]
    fn conversion() {
        let datetime = DateTime::parse_from_rfc3339("2022-08-26T08:13:03.118623+00:00").unwrap();
        println!("{:?}", datetime.naive_local());
    }
}
