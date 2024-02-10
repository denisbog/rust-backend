use ::server::ServerImpl;
use async_session::MemoryStore;
use axum::extract::MatchedPath;
use axum::response::Response;
use axum::routing::get_service;
use axum_server::tls_rustls::RustlsConfig;
use http::{Method, Request};
use index::search::SearchEngine;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use openapi::server;
use sqlx::MySqlPool;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;
use std::{env, sync::Arc};
use tokio::signal;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::{Level, Span};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mariadb://root:my-secret-pw@localhost/items".to_string());
    let pool = MySqlPool::connect(&db_connection_str).await.unwrap();

    let search_engine = Arc::new(SearchEngine::default());
    SearchEngine::start(search_engine.clone()).await;

    let server = ServerImpl {
        store: MemoryStore::new(),
        oauth_client: oauth_client().unwrap(),
        pool,
        search_engine,
    };

    let tracing = TraceLayer::new_for_http()
        .make_span_with(|req: &Request<_>| {
            let path = if let Some(path) = req.extensions().get::<MatchedPath>() {
                path.as_str()
            } else {
                req.uri().path()
            };
            let method = if let Some(method) = req.extensions().get::<Method>() {
                method.as_str()
            } else {
                req.method().as_str()
            };
            tracing::info_span!("http-request", %path, %method)
        })
        .on_response(|response: &Response<_>, latency: Duration, _: &Span| {
            tracing::event!(
                Level::INFO,
                latency = format_args!("{} ms", latency.as_millis()),
                status = &tracing::field::display(response.status()),
                "finished processing request"
            );
        })
        .on_failure(|_: ServerErrorsFailureClass, latency: Duration, _: &Span| {
            tracing::event!(
                Level::ERROR,
                latency = format_args!("{} ms", latency.as_millis()),
                "failure during request processing"
            );
        });

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = server::new(Arc::new(server))
        .route("/", get_service(ServeDir::new("./static")))
        .route_layer(tracing)
        .route_layer(cors);

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
