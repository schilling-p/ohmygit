use std::path::PathBuf;
use axum::extract::{Path, Query, State};
use axum::response::Response;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Basic;
use axum_extra::TypedHeader;
use axum_macros::debug_handler;

use error::AppError;
use domain::request::repository::{InfoRefsQuery, RepoAction};
use shared::state::AppState;
use crate::repository::git::format::format_git_advertisement;
use crate::repository::read::find_repository_by_name;
use crate::repository::git::constants::*;
use crate::repository::git::run::run_git_advertise_refs;
use crate::repository::auth::authenticate_and_authorize_user;
use crate::repository::git::format::build_git_advertisement_response;

// test command: GIT_TRACE_PACKET=1 GIT_TRACE=1 git clone http://0.0.0.0:3001/paul/ohmygit.git
#[debug_handler]
pub async fn handle_info_refs(
    State(app_state): State<AppState>,
    Path((username, repo_name)): Path<(String, String)>,
    Query(query): Query<InfoRefsQuery>,
    opt_auth: Option<TypedHeader<Authorization<Basic>>>) -> Result<Response, AppError> {
    let pool = &app_state.db;
    let possible_operations = [GIT_CLONE_PACK.to_string(), GIT_PUSH_PACK.to_string()];
    if !possible_operations.contains(&query.service) {
        return Err(AppError::BadRequest(format!("Unsupported service: {:?}", query.service)))
    }

    let repo_name_no_git = repo_name.strip_suffix(".git").unwrap_or(&repo_name);
    let repo = find_repository_by_name(&pool, &repo_name_no_git).await?;
    let repo_path = PathBuf::from(format!("{GIT_REPO_PATH}/{}/{}.git", username, repo_name_no_git));

    if repo.is_public && (query.service == GIT_CLONE_PACK) {
        let output = run_git_advertise_refs(&query.service, repo_path).await?;
        let formatted_output = format_git_advertisement(&query.service, &output);
        build_git_advertisement_response(&query.service, formatted_output)

    } else {
        let auth_header = opt_auth.ok_or(AppError::GitUnauthorized("Missing username or password from authorization header".into()))?;
        let action = RepoAction::try_from(query.service.as_str())?;

        authenticate_and_authorize_user(&pool ,auth_header, repo, action).await?;

        let output = run_git_advertise_refs(&query.service, repo_path).await?;
        let formatted_output = format_git_advertisement(&query.service, &output);
        build_git_advertisement_response(&query.service, formatted_output)
    }
}