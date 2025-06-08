use axum::Router;
use axum::routing::{get, post};
use application::user::read::list_users;
use application::user::create::create_user;
use application::user::login::user_web_login_handler;
use infrastructure::diesel::DbPool;
use shared::state::AppState;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/users", get(list_users))
        .route("/signup", post(create_user))
        .route("/login", post(user_web_login_handler))
        .with_state(app_state)
}