use axum::Router;
use axum::routing::{get, post};
use application::user::read::list_users;
use application::user::create::create_user;
use application::user::login::login_user;
use infrastructure::diesel::DbPool;

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route("/users", get(list_users))
        .route("/signup", post(create_user))
        .route("/login", post(login_user))
        .with_state(pool)
}