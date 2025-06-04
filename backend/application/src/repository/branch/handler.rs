use axum::extract::{Path, State};
use axum::extract::Json;
use axum::response::{IntoResponse, Redirect};
use axum_macros::debug_handler;
use tower_sessions::Session;
use domain::ApiResponse;
use domain::request::auth::UserIdentifier;
use error::AppError;
use domain::response::repository::RepositoryBranches;
use domain::request::repository::{AuthorizationRequest, CreateBranchRequest, RepoAction};
use infrastructure::diesel::DbPool;
use infrastructure::git2::GitRepository;
use crate::repository::auth::authorize_repository_action;
use crate::repository::branch::create::write_branch_to_database;
use crate::repository::read::find_repository_by_name;
use crate::user::read::retrieve_user_from_db;

#[debug_handler]
pub async fn list_repository_branches(Path((username, repo_name)): Path<(String, String)>) -> Result<ApiResponse, AppError> {
    // TODO: figure out why this does not check the session, does no auth at all
    let repo_path = format!("/repos/{}/{}.git", username, repo_name);
    let git_repo = GitRepository::open(&repo_path)?;
    let branches = RepositoryBranches {branches: git_repo.list_local_branches()?};
    Ok(ApiResponse::RepositoryBranches(branches))
}

#[debug_handler]
pub async fn create_repository_branch(session: Session, State(pool): State<DbPool>, Path((username, repo_name)): Path<(String, String)>, Json(create_branch_request): Json<CreateBranchRequest>) -> Result<impl IntoResponse, AppError> {
    let Some(current_user) = session.get::<String>("username").await? else {
        return Err(AppError::Unauthorized);
    };

    let user = retrieve_user_from_db(&pool, UserIdentifier::Username(current_user)).await?;
    let repository = find_repository_by_name(&pool, &repo_name).await?;
    let repo_action = RepoAction::CreateBranch;
    let auth_request = AuthorizationRequest {
        user, repository, repo_action,
    };

    authorize_repository_action(&pool, auth_request).await?;

    let repo_path = format!("/repos/{}/{}.git", &username, &repo_name);
    let git_repo = GitRepository::open(&repo_path)?;
    git_repo.create_branch(&create_branch_request.new_branch_name, &create_branch_request.base_branch_name, create_branch_request.switch_head)?;

    write_branch_to_database(&pool, &create_branch_request.new_branch_name).await?;

    let recently_authorized_key = format!("{}:{}", &username, &repo_name);
    session.insert("recently_authorized_repo", recently_authorized_key).await?;

    let redirect_url = format!("/repos/{}/{}/branch/{}", username, repo_name, create_branch_request.new_branch_name);
    Ok(Redirect::to(&redirect_url))
}