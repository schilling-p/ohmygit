use axum::Router;
use axum::routing::get;
use application::templates::dashboard::dashboard_template;
use application::templates::repository::repository_template;
use infrastructure::diesel::DbPool;

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route("/dashboard", get(dashboard_template))
        .route("/repos/{username}/{repository_name}", get(repository_template))
        .with_state(pool)
}