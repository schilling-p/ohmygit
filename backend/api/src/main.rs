use std::time::Duration;
use time::Duration as Tduration;
use axum::response::Response;
use axum::http::Request;
use axum::extract::MatchedPath;
use anyhow::Context;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::{info_span, Span};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tower_http::cors::CorsLayer;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};

use infrastructure::diesel::connection::{init_pool, run_migrations, DbPool};
use shared::graceful::shutdown_signal;
use state::AppState;

mod routes;
mod handlers;

#[tokio::main]
async fn main() -> anyhow::Result<()>{

    // TODO: handle the error case better than with unwrap()
    run_migrations().unwrap();
    let pool = init_pool();
    let app_state = AppState::initialize_app_state(pool);

    // TODO: remove or find better way for production than the current permissive CorsLayer, there is an example in the axum GH repo
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Tduration::seconds(20)));
    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {"debug, tower_http=debug".into()});
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = routes::create_routes(app_state)
        .layer(tower_http::catch_panic::CatchPanicLayer::new())
        .layer(CorsLayer::permissive())
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
        )
        .layer(session_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.context("failed to bind TCP listener")?;
    tracing::debug!("Listening on: {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("axum:serve failed")?;

    Ok(())
}