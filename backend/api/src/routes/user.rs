use axum::Router;
use axum::routing::{get, post};
use crate::handlers::user::{user_sign_up, list_users};
use application::user::login::user_web_login_handler;
use shared::state::AppState;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/users", get(list_users))
        .route("/signup", post(user_sign_up))
        .route("/login", post(user_web_login_handler))
        .with_state(app_state)
}