use std::path::PathBuf;
use axum::extract::{Path, State};
use axum::response::Response;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Basic;
use axum_extra::TypedHeader;
use axum_macros::debug_handler;
use tracing::debug;

use error::AppError;
use domain::request::repository::RepoAction;
use shared::state::AppState;

use crate::repository::read::find_repository_by_name;
use crate::repository::auth::authenticate_and_authorize_user;
use crate::repository::git::{GIT_REPO_PATH, GIT_CLONE_PACK};
use crate::repository::git::run::run_git_pack;
use crate::repository::git::format::build_git_pack_response;

#[debug_handler]
pub async fn send_user_repository(State(app_state): State<AppState>, Path((username, repo_name)): Path<(String, String)>, opt_auth: Option<TypedHeader<Authorization<Basic>>>, body: axum::body::Bytes) -> Result<Response, AppError> {
    let auth_header = opt_auth.ok_or(AppError::GitUnauthorized("Missing username or password from authorization header".into()))?;
    let pool = &app_state.db;
    let repo_name_no_git = repo_name.strip_suffix(".git").unwrap_or(&repo_name);
    let repo_path = PathBuf::from(format!("{GIT_REPO_PATH}/{}/{}", username, repo_name_no_git));
    let repo = find_repository_by_name(&pool, repo_name_no_git).await?;
    authenticate_and_authorize_user(&pool, auth_header, repo, RepoAction::Clone).await?;

    debug!("sending user repository: {:?}", &repo_path);
    let service: &str = GIT_CLONE_PACK;
    let output = run_git_pack(&service, repo_path, body).await?;
    let response = build_git_pack_response(service, output)?;

    Ok(response)
}