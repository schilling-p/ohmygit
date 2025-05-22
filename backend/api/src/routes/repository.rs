use axum::Router;
use axum::routing::{get, post};

use application::repository::read::{handle_info_refs, send_user_repository, receive_user_repository};
use infrastructure::diesel::DbPool;

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route("/{user}/{repo_name.git}/info/refs", get(handle_info_refs))
        .route("/{user}/{repo_name.git}/git-upload-pack", post(send_user_repository))
        .route("/{user}/{repo_name.git}/git-receive-pack", post(receive_user_repository))
        .with_state(pool)
}