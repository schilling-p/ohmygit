use std::time::Duration;
use axum::Router;
use axum::routing::{get, post};
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum::http::Request;
use axum::extract::MatchedPath;
use anyhow::Context;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::{info_span, Span};

use infrastructure::{init_pool, run_migrations};
use application::user::read::list_users;
use application::user::create::create_user;
use application::user::login::login_user;
use domain::models::HealthResponse;
use shared::graceful::shutdown_signal;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()>{

    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {"debug, tower_http=debug".into()});
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    // TODO: handle the error case better than with unwrap()
    run_migrations().unwrap();
    let pool = init_pool();

    let app = Router::new()
        .route("/health", get(healthcheck))
        .route("/new_user", post(create_user))
        .route("/users", get(list_users))
        .route("/login", post(login_user))
        .layer(tower_http::catch_panic::CatchPanicLayer::new())
        // TODO: remove or find better way for production than this CorsLayer
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
            .on_failure(|error: ServerErrorsFailureClass, latency: Duration, span: &Span| {
                tracing::error!(?error, "Request failed after {:?}", latency.as_millis());
            })
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.context("failed to bind TCP listener")?;
    tracing::debug!("Listening on: {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("axum:serve failed")?;

    Ok(())
}

async fn healthcheck() -> impl IntoResponse {
    Json(HealthResponse {message: "Ok"})
}