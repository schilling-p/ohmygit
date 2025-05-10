use std::time::Duration;
use axum::response::Response;
use axum::http::Request;
use axum::extract::MatchedPath;
use anyhow::Context;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::{info_span, Span};
use tower_http::cors::CorsLayer;
use tower_sessions::{MemoryStore, SessionManagerLayer};

use infrastructure::diesel::{init_pool, run_migrations};
use shared::graceful::shutdown_signal;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()>{

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {"debug, tower_http=debug".into()});
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    // TODO: handle the error case better than with unwrap()
    run_migrations().unwrap();
    let pool = init_pool();

    let app = routes::create_routes(pool)
        .layer(tower_http::catch_panic::CatchPanicLayer::new())
        // TODO: remove or find better way for production than this CorsLayer
        .layer(CorsLayer::permissive())
        .layer(session_layer)
        .layer(TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                let matched_path = request.extensions().get::<MatchedPath>().map(MatchedPath::as_str);
                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            })
            .on_request(|request: &Request<_>, span: &Span| {
                if let Some(ip) = request.headers().get("x-forwarded-for") {
                    span.record("client ip: ", &ip.to_str().unwrap_or("unknown"));
                }
            })
            .on_response(|response: &Response<_>, latency: Duration, span: &Span| {
                span.record("status_code", &tracing::field::display(response.status()));
                span.record("latency", &tracing::field::display(response.status()));
                tracing::info!("Finished handling request in {}ms", latency.as_millis());
            })
            .on_failure(|error: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                tracing::error!(?error, "Request failed after {:?}", latency.as_millis());
            })
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.context("failed to bind TCP listener")?;
    tracing::debug!("Listening on: {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("axum:serve failed")?;
 
    Ok(())
}