use axum::Router;
use axum::routing::{get, post};

use application::repository::git::advertise::handle_info_refs;
use application::repository::git::push::receive_user_repository;
use application::repository::git::clone::send_user_repository;
use application::repository::branch::handler::list_repository_branches;
use application::repository::branch::handler::create_repository_branch;
use infrastructure::diesel::DbPool;

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route("/{user}/{repo_name.git}/info/refs", get(handle_info_refs))
        .route("/{user}/{repo_name.git}/git-upload-pack", post(send_user_repository))
        .route("/{user}/{repo_name.git}/git-receive-pack", post(receive_user_repository))
        .route("/repos/{user}/{repo_name}/branches", get(list_repository_branches))
        .route("/repos/{user}/{repo_name}/branches", post(create_repository_branch))
        .with_state(pool)
}