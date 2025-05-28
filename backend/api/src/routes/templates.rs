use axum::Router;
use axum::routing::get;
use application::templates::dashboard::dashboard;
use application::templates::repository::repository;
use infrastructure::diesel::DbPool;

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route("/dashboard", get(dashboard))
        .with_state(pool)
        .merge(repo_route())
}

fn repo_route() -> Router {
    Router::new()
        .route("/repos/{repository_name}", get(repository))
}