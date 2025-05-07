mod health;
mod user;
mod repository;

use axum::Router;

pub fn create_routes(pool: deadpool_diesel::postgres::Pool) -> Router {
    Router::new()
        .merge(health::routes())
        .merge(user::routes(pool.clone()))
        .merge(repository::routes(pool.clone()))
}