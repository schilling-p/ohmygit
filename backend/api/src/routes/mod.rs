mod health;
mod user;
mod repository;
mod templates;

use axum::Router;
use infrastructure::diesel::DbPool;

pub fn create_routes(pool: DbPool) -> Router {
    Router::new()
        .merge(health::routes())
        .merge(user::routes(pool.clone()))
        .merge(repository::routes(pool.clone()))
        .merge(templates::routes(pool))
}