use axum::extract::Path;
use infrastructure::git2::*;
use axum_macros::debug_handler;
use domain::ApiResponse;
use error::AppError;
use domain::response::repository::RepositoryBranches;
#[debug_handler]
pub async fn list_repository_branches(Path((username, repo_name)): Path<(String, String)>) -> Result<ApiResponse, AppError> {
    let repo_path = format!("/repos/{}/{}.git", username, repo_name);
    let git_repo = GitRepository::open(&repo_path)?;
    let branches = RepositoryBranches {branches: git_repo.list_local_branches()?};
    Ok(ApiResponse::RepositoryBranches(branches))
}