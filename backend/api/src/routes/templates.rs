use axum::Router;
use axum::routing::get;
use application::templates::repository::{repository_template_default, repository_template_for_branch, create_repository_template};
use shared::state::AppState;

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/dashboard", get(dashboard_template))
        .route("/repos/{username}/{repository_name}", get(repository_template_default))
        .route("/repos/{username}/{repository_name}/branch/{branch_name}", get(repository_template_for_branch))
        .route("/repos/create", get(create_repository_template))
        .with_state(app_state)
}