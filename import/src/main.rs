use std::{collections::HashMap, io::BufRead, path::PathBuf, sync::Arc};

use chrono::DateTime;
use common::{DbItem, Point};
use index::search::SearchEngine;
use serde_derive::{Deserialize, Serialize};
use serde_dynamo::AttributeValue;
use sqlx::MySqlPool;

#[derive(Default, Debug, Deserialize, Serialize)]
struct Place {
    lat: f64,
    lng: f64,
}

#[derive(Default, Debug, Deserialize, Serialize)]
struct Item {
    id: String,
    title: Option<String>,
    description: Option<String>,
    category: Option<String>,
    subcategory: Option<String>,
    user: Option<String>,
    address: Option<String>,
    #[serde(rename = "priceType")]
    price_type: Option<String>,
    created: Option<String>,
    place: Option<Place>,
    images: Option<Vec<String>>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
struct Wrap {
    #[serde(rename = "Item")]
    item: HashMap<String, AttributeValue>,
}
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let items: Vec<Item> = std::io::BufReader::new(
        std::fs::File::open(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("all.json")).unwrap(),
    )
    .lines()
    .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
    .map(|item: Wrap| serde_dynamo::from_item(serde_dynamo::Item::from(item.item)).unwrap())
    .collect();
    let datetime = DateTime::parse_from_rfc3339("2022-08-26T08:13:03.118623+00:00").unwrap();
    println!("{:?}", datetime.naive_local());
    println!("{:?}", items.get(0));
    let search_engine = Arc::new(SearchEngine::default());

    search_engine.reset_index().await;
    SearchEngine::start(search_engine.clone()).await;

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mariadb://root:my-secret-pw@localhost/items".to_string());
    let pool = MySqlPool::connect(&db_connection_str).await.unwrap();
    futures::future::join_all(items.into_iter().map(|item| async {
        // DbItem {
        //     id: Some(item.id),
        //     title: item.title,
        //     description: item.description,
        //     category: item.category,
        //     subcategory: item.subcategory,
        //     created: None,
        //     location: Some(Point {
        //         lat: item.place.as_ref().unwrap().lat,
        //         lng: item.place.as_ref().unwrap().lng,
        //     }),
        //     price: Some(100f64),
        //     place_description: item.address,
        //     price_type: item.price_type,
        //     reserved: None,
        //     status: None,
        //     updated: None,
        //     user: Some("genered".into()),
        // };
        store(&pool, &search_engine, item).await;
    }))
    .await;
    search_engine.commit().await;
}

async fn store(pool: &MySqlPool, search_engine: &SearchEngine, item: Item) {
    if let Ok(mut transaction) = pool.begin().await {
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
        user,
        status
        )
    VALUES (?, ?, ?, ?, Point(?,?), ?, ?, ?, ?, ?)
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
                "generated",
                "all",
            )
            .execute(&mut *transaction)
            .await
            .unwrap()
            .last_insert_id();

            tracing::info!("{id}");

            let db_item = DbItem {
                id: Some(id),
                title: item.title,
                description: item.description,
                category: item.category,
                subcategory: item.subcategory,
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
        }
        let _ = transaction.commit().await;
    };
}
