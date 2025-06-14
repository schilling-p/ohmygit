mod health;
mod user;
mod repository;
mod templates;

use axum::Router;
use state::AppState;

pub fn create_routes(app_state: AppState) -> Router {
    Router::new()
        .merge(health::routes())
        .merge(user::routes(app_state.clone()))
        .merge(repository::routes(app_state.clone()))
        .merge(templates::routes(app_state.clone()))
}