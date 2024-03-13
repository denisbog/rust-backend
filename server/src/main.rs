use ::server::ServerImpl;
use async_session::MemoryStore;
use axum::extract::{Host, MatchedPath};
use axum::handler::HandlerWithoutStateExt;
use axum::response::Response;
use axum::routing::get_service;
use axum::BoxError;
use axum_server::tls_rustls::RustlsConfig;
use futures::Future;
use http::header::{AUTHORIZATION, CONTENT_TYPE, COOKIE};
use http::{Method, Request, Uri};
use index::search::SearchEngine;
use openapi::server;
use sqlx::MySqlPool;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;
use std::{env, sync::Arc};
use tokio::signal;
use tokio::sync::RwLock;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::{Level, Span};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db_connection_str = std::env::var("DATABASE_URL").unwrap();
    let pool = MySqlPool::connect(&db_connection_str).await.unwrap();

    let search_engine = Arc::new(SearchEngine::default());
    SearchEngine::start(search_engine.clone()).await;

    let server = ServerImpl {
        store: MemoryStore::new(),
        oauth_client: ServerImpl::oauth_client().unwrap(),
        cache: RwLock::new(HashMap::new()),
        pool,
        search_engine: search_engine.clone(),
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
        .allow_headers([AUTHORIZATION, COOKIE, CONTENT_TYPE])
        .allow_origin([
            "https://localhost:3000".parse().unwrap(),
            "https://example.com:3000".parse().unwrap(),
            "https://rest.com:3443".parse().unwrap(),
        ])
        .allow_credentials(true);

    let static_web_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static");

    tracing::info!("serve web content from folder {:?}", static_web_folder);
    let app = server::new(Arc::new(server))
        .route_layer(tracing)
        .route_layer(cors)
        .fallback_service(get_service(ServeDir::new(static_web_folder)));

    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("domain.crt"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("domain.key"),
    )
    .await
    .unwrap();
    let http_port = std::env::var("HTTP").unwrap_or_else(|_| "80".to_string());
    let https_port = std::env::var("HTTPS").unwrap_or_else(|_| "443".to_string());

    let addr = SocketAddr::from(([0, 0, 0, 0], https_port.parse().unwrap()));
    tracing::debug!("listening on {}", addr);

    let handle = axum_server::Handle::new();
    let shutdown_future = shutdown_signal(handle.clone());
    tokio::spawn(redirect_http_to_https(
        http_port,
        https_port,
        shutdown_future,
    ));

    axum_server::bind_rustls(addr, config)
        .handle(handle)
        .serve(app.into_make_service())
        .await
        .unwrap();

    search_engine.close().await;
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

async fn redirect_http_to_https<F>(http_port: String, https_port: String, signal: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    fn make_https(source: String, target: String, host: String, uri: Uri) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&source, &target);
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let source_clone = http_port.clone();
    let redirect = |Host(host): axum::extract::Host, uri: axum::http::Uri| async move {
        match make_https(source_clone, https_port, host, uri) {
            Ok(uri) => Ok(axum::response::Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                Err(axum::http::StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], http_port.parse().unwrap()));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {addr}");
    axum::serve(listener, redirect.into_make_service())
        .with_graceful_shutdown(signal)
        .await
        .unwrap();
}
