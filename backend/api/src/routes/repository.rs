use axum::Router;
use axum::routing::{post};
use application::repository::read::{get_repository, list_user_repositories};

pub fn routes(pool: deadpool_diesel::postgres::Pool) -> Router {
    Router::new()
        .route("/repos/{username}/{repository_name}", post(get_repository))
        .route("/user_repositories", post(list_user_repositories))
        .with_state(pool)
}