use axum::Router;
use axum::routing::get;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use anyhow::Context;
use tokio::signal;
use infrastructure::{init_pool, run_migrations};
use application::user::read::list_users;
use tower_http::cors::CorsLayer;


// https://github.com/tokio-rs/axum/blob/main/examples/anyhow-error-response/src/main.rs
struct AppError(anyhow::Error);

#[derive(serde::Serialize)]
struct HealthResponse {
    message: &'static str,
}

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    // TODO: handle the error case better than with unwrap()
    run_migrations().unwrap();
    let pool = init_pool();

    let app = Router::new()
        .route("/health", get(healthcheck))
        .route("/users", get(list_users))
        .layer(tower_http::catch_panic::CatchPanicLayer::new())
        // TODO: remove or find better way for production than this CorsLayer
        .layer(CorsLayer::permissive())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.context("failed to bind TCP listener")?;
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