use axum::Router;
use axum::routing::get;
use application::templates::dashboard::dashboard;

pub fn routes(pool: deadpool_diesel::postgres::Pool) -> Router {
    Router::new()
        .route("/dashboard", get(dashboard))
        .with_state(pool)
}