use std::time::Duration;
use axum::Router;
use axum::routing::{get, post};
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum::http::Request;
use axum::extract::MatchedPath;
use anyhow::Context;
use tokio::signal;
use infrastructure::{init_pool, run_migrations};
use application::user::read::list_users;
use application::user::create::create_user;
use application::user::login::login_user;
use tower_http::cors::CorsLayer;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::{info_span, Span};

#[derive(serde::Serialize)]
struct HealthResponse {
    message: &'static str,
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                ).into()
            }),
        )
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
    tracing::debug!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("axum:serve failed")?;

    println!("Server has started");

    Ok(())
}

// https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs
async fn shutdown_signal() {
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
}

async fn healthcheck() -> impl IntoResponse {
    Json(HealthResponse {message: "Ok"})
}