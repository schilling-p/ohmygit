use axum::Router;
use axum::routing::get;
use state::AppState;

use crate::handlers::template::{dashboard_template, repository_creation_template};

pub fn routes(app_state: AppState) -> Router {
    Router::new()
        .route("/dashboard", get(dashboard_template))
        .route("/repos/{username}/{repository_name}", get(repository_template_default))
        .route("/repos/{username}/{repository_name}/branch/{branch_name}", get(repository_template_for_branch))
        .route("/repos/create", get(repository_creation_template))
        .with_state(app_state)
}