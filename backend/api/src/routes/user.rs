use axum::Router;
use axum::routing::{get, post};
use crate::handlers::user::{user_web_signup_handler, list_users, user_web_login_handler};
use state::AppState;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/users", get(list_users))
        .route("/signup", post(user_web_signup_handler))
        .route("/login", post(user_web_login_handler))
        .with_state(app_state)
}