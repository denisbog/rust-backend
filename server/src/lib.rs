use std::{io::Cursor, sync::Arc};

use async_session::{MemoryStore, SessionStore};
use axum_extra::extract::CookieJar;
use bytes::Bytes;
use common::{DbItem, DbUser};
use index::search::SearchEngine;
use oauth2::basic::BasicClient;
use openapi::models::{Item, ItemPlace, User};
use sqlx::{FromRow, MySql, MySqlPool, QueryBuilder, Row};

mod server_impl;
use tokio::io::AsyncReadExt;
pub struct ServerImpl {
    pub store: MemoryStore,
    pub oauth_client: BasicClient,
    pub pool: MySqlPool,
    pub search_engine: Arc<SearchEngine>,
}
impl ServerImpl {
    pub async fn get_session_user_id(&self, cookie: &CookieJar) -> Option<String> {
        if let Some(session_cookie) = cookie.get("session") {
            if let Ok(Some(user_data)) =
                self.store.load_session(session_cookie.value().into()).await
            {
                let user: User = user_data.get("user").unwrap();
                tracing::info!("user {:?}", user);
                user.id
            } else {
                None
            }
        } else {
            None
        }
    }

    pub async fn check_and_import_user_if_required(&self, user: &User) {
        let mut query_builder = QueryBuilder::new("SELECT name FROM users WHERE id =");
        query_builder.push_bind(&user.id);
        let query = query_builder.build();

        if query.fetch_optional(&self.pool).await.unwrap().is_none() {
            tracing::info!("inserting new user in the database {:?}", user);
            sqlx::query!(
                "insert into users (id, name, email) values (?, ?, ?)",
                user.id,
                user.name,
                user.email
            )
            .execute(&self.pool)
            .await
            .unwrap();
        } else {
            sqlx::query!(
                "update users set last_login = current_time() where id = ?",
                user.id
            )
            .execute(&self.pool)
            .await
            .unwrap();
        }
    }

    pub async fn get_items(
        &self,
        category: Option<String>,
        subcategory: Option<String>,
        lat: Option<f64>,
        long: Option<f64>,
        r: Option<f64>,
        q: Option<String>,
        user: Option<String>,
        reserved: Option<String>,
        last_evaluated_key: Option<String>,
    ) -> (Vec<Item>, Option<String>) {
        let query = r#"
    SELECT
        items.id,
        items.title,
        items.description,
        items.created,
        items.updated,
        items.price_type,
        items.price,
        items.location,
        items.place_description,
        items.category,
        items.subcategory,
        items.user,
        items.reserved,
        items.status,
        users.name,
        users.email,
        users.avatar,
        users.joined,
        users.last_login
    FROM 
        items
    LEFT JOIN
        users
    ON
        items.user = users.id"#;

        let mut builder = QueryBuilder::<MySql>::new(query);

        let mut append_condition = false;

        if let Some(ref category) = category {
            if !append_condition {
                builder.push(" WHERE");
                append_condition = true;
            }
            builder.push(" category = ");
            builder.push_bind(category);
            if let Some(ref subcategory) = subcategory {
                builder.push(" and subcategory in (");
                let mut separated = builder.separated(',');
                subcategory.split(',').for_each(|subcategory| {
                    separated.push_bind(subcategory);
                });
                separated.push_unseparated(')');
            }
        }

        if let (Some(lat), Some(lng), Some(r)) = (lat, long, r) {
            if append_condition {
                builder.push(" AND");
            } else {
                builder.push(" WHERE");
                append_condition = true;
            }
            builder.push(" ST_DISTANCE_SPHERE(location, POINT(");
            builder.push_bind(lat);
            builder.push(", ");
            builder.push_bind(lng);
            builder.push(")) < ");
            builder.push_bind(r);
        }

        if let Some(user) = user {
            if append_condition {
                builder.push(" AND");
            } else {
                builder.push(" WHERE");
                append_condition = true;
            }
            builder.push(" items.user = ");
            builder.push_bind(user);
        }

        if let Some(reserved) = reserved {
            if append_condition {
                builder.push(" AND");
            } else {
                builder.push(" WHERE");
                append_condition = true;
            }
            builder.push(" items.reserved = ");
            builder.push_bind(reserved);
        }

        if let Some(q) = q {
            if append_condition {
                builder.push(" AND");
            } else {
                builder.push(" WHERE");
                append_condition = true;
            }
            builder.push(" MATCH(title, description) AGAINST(");
            builder.push_bind(q);
            builder.push(")");
        }

        if let Some(last_id) = last_evaluated_key {
            if append_condition {
                builder.push(" AND");
            } else {
                builder.push(" WHERE");
                // append_condition = true;
            }
            builder.push(" items.id <= ");
            builder.push_bind(last_id);
        }
        builder.push(" ORDER BY id DESC");
        builder.push(" LIMIT 11");
        // println!("{}", builder.sql());
        let execute_query = builder.build();
        let recs = execute_query.fetch_all(&self.pool).await.unwrap();

        let last_evaluated_key: Option<String> = if recs.len() > 10 {
            let last_id: Option<u64> = recs.last().unwrap().try_get("id").unwrap();
            Some(last_id.unwrap().to_string())
        } else {
            None
        };

        let items = recs
            .into_iter()
            .take(10)
            .map(|row| {
                let rec = DbItem::from_row(&row).unwrap();
                let mut item = Item::new();
                item.id = Some(rec.id.unwrap().to_string());
                item.title = rec.title;
                item.description = rec.description;

                if let Some(native_date_time) = rec.created {
                    item.created = Some(native_date_time.and_utc());
                }
                if let Some(native_date_time) = rec.updated {
                    item.updated = Some(native_date_time.and_utc());
                }

                item.price_type = rec.price_type;
                item.price = rec.price;

                if let Some(coordinates) = rec.location {
                    item.place = Some(ItemPlace {
                        lat: Some(coordinates.lat),
                        lng: Some(coordinates.lng),
                        description: rec.place_description,
                    })
                }
                item.category = rec.category;
                item.subcategory = rec.subcategory;

                item.user = rec.user;
                item.reserved = rec.reserved;

                item.status = rec.status;

                item.user_name = row.try_get("name").unwrap();
                item.user_email = row.try_get("email").unwrap();
                item.user_avatar = row.try_get("avatar").unwrap();
                item
            })
            .collect::<Vec<Item>>();

        (items, last_evaluated_key)
    }

    pub async fn get_images_for_item(&self, id: &str) -> Option<Vec<String>> {
        if let Ok(mut contents) = tokio::fs::read_dir(format!("content/{}", id)).await {
            let mut items = Vec::<String>::new();
            while let Some(content_folder) = contents.next_entry().await.unwrap() {
                items.push(content_folder.file_name().into_string().unwrap());
            }
            return Some(items);
        }
        None
    }

    pub async fn upload_content(&self, id: &str, file_name: &str, bytes: &Bytes) {
        let file_name = if file_name.contains('.') {
            file_name.rsplit_once('.').unwrap().0
        } else {
            file_name
        };
        let dest = format!("content/{}/{}.jpeg", id, file_name);

        tokio::fs::create_dir_all(format!("content/{}", id))
            .await
            .unwrap();

        let from_bytes = Cursor::new(bytes);
        let image = image::io::Reader::new(from_bytes)
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();
        let image = image.resize(720, 720, image::imageops::FilterType::CatmullRom);
        image
            .save_with_format(dest, image::ImageFormat::Jpeg)
            .unwrap();
    }

    pub async fn get_content(&self, id: &str, name: &str) -> Option<Vec<u8>> {
        if let Ok(mut file) = tokio::fs::File::open(format!("./content/{}/{}", id, name)).await {
            let mut contents = vec![];
            file.read_to_end(&mut contents).await.unwrap();
            Some(contents)
        } else {
            None
        }
    }

    pub async fn delete_content(&self, id: &str, name: &str) -> Result<(), std::io::Error> {
        tokio::fs::remove_file(format!("./content/{}/{}", id, name)).await
    }

    pub async fn move_images(&self, session_id: &str, id: &str) -> Result<(), std::io::Error> {
        tokio::fs::rename(
            format!("./content/{}", session_id),
            format!("./content/{}", id),
        )
        .await
    }
    pub fn db_to_rest_user(&self, db_user: DbUser) -> User {
        User {
            id: db_user.id,
            name: db_user.name,
            about: db_user.about,
            avatar: db_user.avatar,
            email: db_user.email,
            joined: db_user
                .joined
                .map(|native_date_time| native_date_time.and_utc()),
            last_login: db_user
                .last_login
                .map(|native_date_time| native_date_time.and_utc()),
        }
    }
}
