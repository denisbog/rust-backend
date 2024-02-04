use std::{collections::HashMap, io::BufRead, path::PathBuf};

use chrono::DateTime;
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
    let items: Vec<Item> = std::io::BufReader::new(
        std::fs::File::open(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("all.json")
                .iter(),
        )
        .unwrap(),
    )
    .lines()
    .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
    .map(|item: Wrap| serde_dynamo::from_item(serde_dynamo::Item::from(item.item)).unwrap())
    .collect();
    let datetime = DateTime::parse_from_rfc3339("2022-08-26T08:13:03.118623+00:00").unwrap();
    println!("{:?}", datetime.naive_local());
    println!("{:?}", items.get(0));

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mariadb://root:my-secret-pw@localhost/items".to_string());
    let pool = MySqlPool::connect(&db_connection_str).await.unwrap();
    futures::future::join_all(items.iter().map(|item| async {
        store(&pool, item).await;
    }))
    .await;
}

async fn store(pool: &MySqlPool, item: &Item) {
    if let Ok(mut transaction) = pool.begin().await {
        match (&item.title, &item.place, &item.category, &item.subcategory) {
            (Some(title), Some(place), Some(category), Some(subcategory)) => {
                sqlx::query_scalar!(
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
    VALUES (?, ?, ?, ?, Point(?,?), ?, ?, ?, ?,?)
    returning id
            "#,
                    item.title,
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
            }
            _ => {}
        };
        let _ = transaction.commit().await;
    };
}
