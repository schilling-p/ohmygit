use axum::Router;
use axum::routing::{post};
use application::repository::read::{get_repository};

pub fn routes(pool: deadpool_diesel::postgres::Pool) -> Router {
    Router::new()
        .route("/repos/{username}/{repository_name}", post(get_repository))
        .with_state(pool)
}