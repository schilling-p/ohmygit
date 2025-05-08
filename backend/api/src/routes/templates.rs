use axum::Router;
use axum::routing::get;
use application::templates::dashboard::dashboard;

pub fn routes() -> Router {
    Router::new()
        .route("/dashboard/{username}", get(dashboard))
}