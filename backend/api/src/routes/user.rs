use axum::Router;
use axum::routing::{get, post};
use application::user::read::list_users;
use application::user::create::create_user;
use application::user::login::user_login_handler;
use infrastructure::diesel::DbPool;

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route("/users", get(list_users))
        .route("/signup", post(create_user))
        .route("/login", post(user_login_handler))
        .with_state(pool)
}