use std::sync::Arc;

use async_session::{MemoryStore, SessionStore};
use axum_extra::extract::CookieJar;
use index::search::SearchEngine;
use oauth2::basic::BasicClient;
use openapi::models::User;
use sqlx::{MySqlPool, QueryBuilder};

mod server_impl;

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
}
