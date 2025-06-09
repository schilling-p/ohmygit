use axum::Router;
use axum::routing::{get, post};

use crate::handlers::repository::{ handle_info_refs, send_user_repository, receive_user_repository, create_repository, list_repository_branches};
use application::repository::branch::handler::create_repository_branch;
use state::AppState;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/{user}/{repo_name.git}/info/refs", get(handle_info_refs))
        .route("/{user}/{repo_name.git}/git-upload-pack", post(send_user_repository))
        .route("/{user}/{repo_name.git}/git-receive-pack", post(receive_user_repository))
        .route("/repos/{user}/{repo_name}/branches", get(list_repository_branches))
        .route("/repos/{user}/{repo_name}/branches", post(create_repository_branch))
        .route("/repos/create", post(create_repository))
        .with_state(app_state)
}