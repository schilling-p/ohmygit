use axum::Json;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use domain::models::HealthResponse;


pub fn routes() -> Router {
    Router::new().route("/health", get(healthcheck))
}
async fn healthcheck() -> impl IntoResponse {
    Json(HealthResponse {message: "Ok"})
}