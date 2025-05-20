use axum::Router;
use axum::routing::get;
use domain::ApiResponse;
use domain::response::health::HealthResponse;

pub fn routes() -> Router {
    Router::new().route("/health", get(healthcheck))
}
async fn healthcheck() -> ApiResponse {
    ApiResponse::Health(HealthResponse {
        message: "Ok",
    })
}