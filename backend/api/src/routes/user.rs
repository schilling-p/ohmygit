use axum::Router;
use axum::routing::{get, post};
use application::user::read::list_users;
use application::user::create::create_user;
use application::user::login::login_user;
use application::repository::read::list_user_repositories;

pub fn routes(pool: deadpool_diesel::postgres::Pool) -> Router {
    Router::new()
        .route("/users", get(list_users))
        .route("/signup", post(create_user))
        .route("/login", post(login_user))
        .route("/user_repositories", post(list_user_repositories))
        .with_state(pool)
}