use async_session::{MemoryStore, Session, SessionStore};
use async_trait::async_trait;
use axum::extract::Host;
use axum_extra::extract::CookieJar;
use axum_server::tls_rustls::RustlsConfig;
use bytes::Bytes;
use common::DbItem;
use common::Point;
use http::Method;
use oauth2::TokenResponse;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
};
use openapi::models::{self, ItemPlace, User};
use openapi::{models::Item, server, ItemsIdContentPutResponse};
use openapi::{
    AuthorizedGetResponse, ItemsGetResponse, ItemsIdContentGetResponse, ItemsIdDeleteResponse,
    ItemsIdGetResponse, ItemsIdPostResponse, ItemsPutResponse, LoginGetResponse,
    MyItemsGetResponse, MyRelatedGetResponse, ReservationsGetResponse,
    ReservationsIdAcceptPostResponse, ReservationsIdDeclinePostResponse,
    ReservationsIdDeleteResponse, ReservationsIdGetResponse, ReservationsIdPostResponse,
    ReservationsIdReturnPostResponse, ReservationsPutResponse, UsersGetResponse,
    UsersIdDeleteResponse, UsersIdGetResponse, UsersIdPostResponse,
};
use sqlx::MySqlPool;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;
use std::{env, sync::Arc};
use tokio::signal;
struct ServerImpl {
    store: MemoryStore,
    oauth_client: BasicClient,
    pool: MySqlPool,
}

impl ServerImpl {
    async fn get_session_user_id(self: &Self, cookie: &CookieJar) -> Option<String> {
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

#[allow(unused_variables)]
#[async_trait]
impl openapi::Api for ServerImpl {
    #[doc = r" ItemsIdContentGet - GET /api/items/{id}/content"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_id_content_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ItemsIdContentGetPathParams,
    ) -> Result<ItemsIdContentGetResponse, String> {
        todo!()
    }

    #[doc = r" ItemsIdDelete - DELETE /api/items/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_id_delete(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ItemsIdDeletePathParams,
    ) -> Result<ItemsIdDeleteResponse, String> {
        todo!()
    }

    #[doc = r" ItemsIdPost - POST /api/items/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_id_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ItemsIdPostPathParams,
        body: models::Item,
    ) -> Result<ItemsIdPostResponse, String> {
        todo!()
    }

    #[doc = r" MyItemsGet - GET /api/my-items"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn my_items_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::MyItemsGetQueryParams,
    ) -> Result<MyItemsGetResponse, String> {
        let recs = sqlx::query!(
            r#"
    SELECT subcategory
    FROM items
            "#
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();

        Ok(openapi::MyItemsGetResponse::Status200(
            openapi::models::Items::new(
                recs.into_iter()
                    .map(|rec| {
                        let mut item = Item::new();
                        item.subcategory = Some(rec.subcategory);
                        item
                    })
                    .collect(),
            ),
        ))
    }

    #[doc = r" MyRelatedGet - GET /api/my-related"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn my_related_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::MyRelatedGetQueryParams,
    ) -> Result<MyRelatedGetResponse, String> {
        todo!()
    }

    #[doc = r" ReservationsGet - GET /api/reservations"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn reservations_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::ReservationsGetQueryParams,
    ) -> Result<ReservationsGetResponse, String> {
        todo!()
    }

    #[doc = r" ReservationsIdAcceptPost - POST /api/reservations/{id}/accept"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn reservations_id_accept_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ReservationsIdAcceptPostPathParams,
    ) -> Result<ReservationsIdAcceptPostResponse, String> {
        todo!()
    }

    #[doc = r" ReservationsIdDeclinePost - POST /api/reservations/{id}/decline"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn reservations_id_decline_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ReservationsIdDeclinePostPathParams,
    ) -> Result<ReservationsIdDeclinePostResponse, String> {
        todo!()
    }

    #[doc = r" ReservationsIdDelete - DELETE /api/reservations/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn reservations_id_delete(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ReservationsIdDeletePathParams,
    ) -> Result<ReservationsIdDeleteResponse, String> {
        todo!()
    }

    #[doc = r" ReservationsIdGet - GET /api/reservations/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn reservations_id_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ReservationsIdGetPathParams,
    ) -> Result<ReservationsIdGetResponse, String> {
        todo!()
    }

    #[doc = r" ReservationsIdPost - POST /api/reservations/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn reservations_id_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ReservationsIdPostPathParams,
        body: models::Reservation,
    ) -> Result<ReservationsIdPostResponse, String> {
        todo!()
    }

    #[doc = r" ReservationsIdReturnPost - POST /api/reservations/{id}/return"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn reservations_id_return_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ReservationsIdReturnPostPathParams,
    ) -> Result<ReservationsIdReturnPostResponse, String> {
        todo!()
    }

    #[doc = r" ReservationsPut - PUT /api/reservations"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn reservations_put(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::Reservation,
    ) -> Result<ReservationsPutResponse, String> {
        todo!()
    }

    #[doc = r" UsersGet - GET /api/users"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn users_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::UsersGetQueryParams,
    ) -> Result<UsersGetResponse, String> {
        todo!()
    }

    #[doc = r" UsersIdDelete - DELETE /api/users/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn users_id_delete(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::UsersIdDeletePathParams,
    ) -> Result<UsersIdDeleteResponse, String> {
        todo!()
    }

    #[doc = r" UsersIdGet - GET /api/users/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn users_id_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::UsersIdGetPathParams,
    ) -> Result<UsersIdGetResponse, String> {
        todo!()
    }

    #[doc = r" UsersIdPost - POST /api/users/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn users_id_post(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::UsersIdPostPathParams,
    ) -> Result<UsersIdPostResponse, String> {
        todo!()
    }

    #[doc = r" AuthorizedGet - GET /api/authorized"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn authorized_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::AuthorizedGetQueryParams,
    ) -> Result<AuthorizedGetResponse, String> {
        let token = self
            .oauth_client
            .exchange_code(AuthorizationCode::new(query_params.code))
            .request_async(async_http_client)
            .await
            .context("failed in sending request request to authorization server")
            .unwrap();

        let client = reqwest::Client::new();
        let user_data: User = client
            .get("https://graph.facebook.com/v19.0/me?fields=name,first_name,last_name,email")
            .bearer_auth(token.access_token().secret())
            .send()
            .await
            .context("failed in sending request to target Url")
            .unwrap()
            .json::<User>()
            .await
            .context("failed to deserialize response as JSON")
            .unwrap();

        let mut session = Session::new();
        session.insert("user", &user_data).unwrap();

        // Store session and get corresponding cookie
        let cookie = &self.store.store_session(session).await.unwrap().unwrap();

        Ok(openapi::AuthorizedGetResponse::Status302 {
            location: Some("/api/items".into()),
            set_cookie: Some(format!("session={cookie}; SameSite=Lax; Path=/")),
        })
    }

    #[doc = r" ItemsGet - GET /api/items"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        query_params: models::ItemsGetQueryParams,
    ) -> Result<ItemsGetResponse, String> {
        if let Some(current_user_id) = cookies.get("session") {
            self.get_session_user_id(&cookies).await;
        }
        let recs = sqlx::query_as!(
            DbItem,
            r#"
    SELECT
        id,
        title,
        description,
        created,
        updated,
        price_type,
        price,
        location "location: Point",
        place_description,
        category,
        subcategory,
        user,
        reserved,
        status
    FROM 
        items
    order by id desc
            "#
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();

        Ok(openapi::ItemsGetResponse::Status200(
            openapi::models::Items::new(
                recs.into_iter()
                    .map(|rec| {
                        let mut item = Item::new();
                        item.id = rec.id;
                        item.name = rec.title;
                        item.title = rec.description;

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
                        item
                    })
                    .collect(),
            ),
        ))
    }

    #[doc = r" ItemsIdContentPut - PUT /api/items/{id}/content"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_id_content_put(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ItemsIdContentPutPathParams,
        body: Bytes,
    ) -> Result<ItemsIdContentPutResponse, String> {
        todo!()
    }

    #[doc = r" ItemsIdGet - GET /api/items/{id}"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_id_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        path_params: models::ItemsIdGetPathParams,
    ) -> Result<ItemsIdGetResponse, String> {
        Ok(openapi::ItemsIdGetResponse::Status200(
            openapi::models::Item::new(),
        ))
    }

    #[doc = r" ItemsPut - PUT /api/items"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn items_put(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        body: models::Item,
    ) -> Result<ItemsPutResponse, String> {
        let session = cookies.get("session").unwrap();
        let current_user_id = self.get_session_user_id(&cookies).await;
        let place = body.place.unwrap();
        if let Ok(mut transaction) = self.pool.begin().await {
            let item_id = sqlx::query_scalar!(
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
        reserved,
        status
        )
    VALUES (?, ?, ?, ?, Point(?,?), ?, ?, ?, ?, ?, ?)
    returning id
            "#,
                body.name,
                body.title,
                body.price_type,
                body.price,
                place.lat,
                place.lng,
                place.description,
                body.category,
                body.subcategory,
                current_user_id,
                body.reserved,
                body.status
            )
            .execute(&mut *transaction)
            .await
            .unwrap()
            .last_insert_id();
            let _ = transaction.commit().await;
            Ok(ItemsPutResponse::Status200(format!("{item_id}")))
        } else {
            Ok(ItemsPutResponse::Status200(format!(
                "failed to get transaction"
            )))
        }
    }

    #[doc = r" LoginGet - GET /api/login"]
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    async fn login_get(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
    ) -> Result<LoginGetResponse, String> {
        if let Some(current_user_id) = self.get_session_user_id(&cookies).await {
            tracing::info!("user session found {current_user_id}");
            Ok(openapi::LoginGetResponse::Status302 {
                location: Some("/api/items".into()),
            })
        } else {
            let (auth_url, _csrf_token) = self
                .oauth_client
                .authorize_url(CsrfToken::new_random)
                .add_scope(Scope::new("email".to_string()))
                .url();

            Ok(openapi::LoginGetResponse::Status302 {
                location: Some(auth_url.to_string()),
            })
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mariadb://root:my-secret-pw@localhost/items".to_string());
    let pool = MySqlPool::connect(&db_connection_str).await.unwrap();

    let app = server::new(Arc::new(ServerImpl {
        store: MemoryStore::new(),
        oauth_client: oauth_client().unwrap(),
        pool,
    }));
    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("domain.crt"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("domain.key"),
    )
    .await
    .unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3443));
    tracing::debug!("listening on {}", addr);

    let handle = axum_server::Handle::new();
    let _shutdown_future = shutdown_signal(handle.clone());
    // tokio::spawn(redirect_http_to_https(shutdown_future));

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

use anyhow::Context;
#[derive(Debug)]
struct AppError(anyhow::Error);

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

fn oauth_client() -> Result<BasicClient, AppError> {
    let client_id = env::var("CLIENT_ID").context("Missing CLIENT_ID!")?;
    let client_secret = env::var("CLIENT_SECRET").context("Missing CLIENT_SECRET!")?;
    let redirect_url = env::var("REDIRECT_URL")
        .unwrap_or_else(|_| "https://localhost.localdomain:3443/api/authorized".to_string());

    let auth_url = env::var("AUTH_URL")
        .unwrap_or_else(|_| "https://www.facebook.com/v19.0/dialog/oauth".to_string());

    let token_url = env::var("TOKEN_URL")
        .unwrap_or_else(|_| "https://graph.facebook.com/v19.0/oauth/access_token".to_string());

    Ok(BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).context("failed to create new authorization server URL")?,
        Some(TokenUrl::new(token_url).context("failed to create new token endpoint URL")?),
    )
    .set_redirect_uri(
        RedirectUrl::new(redirect_url).context("failed to create new redirection URL")?,
    ))
}

async fn shutdown_signal(handle: axum_server::Handle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Received termination signal shutting down");
    handle.graceful_shutdown(Some(Duration::from_secs(10)));
}

// async fn redirect_http_to_https<F>(signal: F)
// where
//     F: Future<Output = ()> + Send + 'static,
// {
//     fn make_https(host: String, uri: Uri) -> Result<Uri, BoxError> {
//         let mut parts = uri.into_parts();
//
//         parts.scheme = Some(axum::http::uri::Scheme::HTTPS);
//
//         if parts.path_and_query.is_none() {
//             parts.path_and_query = Some("/".parse().unwrap());
//         }
//
//         let https_host = host.replace("3000", "3443");
//         parts.authority = Some(https_host.parse()?);
//
//         Ok(Uri::from_parts(parts)?)
//     }
//
//     let redirect = |Host(host): axum::extract::Host, uri: axum::http::Uri| async move {
//         match make_https(host, uri) {
//             Ok(uri) => Ok(axum::response::Redirect::permanent(&uri.to_string())),
//             Err(error) => {
//                 tracing::warn!(%error, "failed to convert URI to HTTPS");
//                 Err(axum::http::StatusCode::BAD_REQUEST)
//             }
//         }
//     };
//
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
//     tracing::debug!("listening on {addr}");
//     axum::serve(listener, redirect.into_make_service())
//         .with_graceful_shutdown(signal)
//         .await
//         .unwrap();
// }
