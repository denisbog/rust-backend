use std::sync::Arc;

use async_session::{MemoryStore, SessionStore};
use axum_extra::extract::CookieJar;
use index::search::SearchEngine;
use oauth2::basic::BasicClient;
use openapi::models::User;
use sqlx::MySqlPool;

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
}
