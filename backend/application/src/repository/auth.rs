use domain::models::{User, Repository};
use domain::request::repository::{AuthorizationRequest, RepoAction};
use infrastructure::diesel::DbPool;
use crate::user::read::get_user_role_for_repository;
use error::AppError;
use uuid::Uuid;

pub async fn authorize_repository_action(pool: &DbPool, auth_request: AuthorizationRequest) -> Result<(), AppError> {
    let user_id: Uuid = auth_request.user.id;
    let repo_id: Uuid = auth_request.repo.id;
    let role = get_user_role_for_repository(pool, user_id, repo_id).await?;
    if let Some(role) = role {
        match (auth_request.action, role.as_str()) {
            (RepoAction::Clone, "reader" | "developer" | "maintainer" | "owner") => Ok(()),
            (RepoAction::Push, "developer" | "maintainer" | "owner") => Ok(()),
            (RepoAction::OpenIssue, "developer" | "maintainer" | "owner") => Ok(()),
            (RepoAction::CommentOnIssue, "developer" | "maintainer" | "owner") => Ok(()),
            (RepoAction::CloseIssue, "developer" | "maintainer" | "owner") => Ok(()),
            (RepoAction::CreateMergeRequest, "developer" | "maintainer" | "owner") => Ok(()),
            (RepoAction::ApproveMergeRequest, "developer" | "maintainer" | "owner") => Ok(()),
            (RepoAction::ManageRepoSettings, "maintainer" | "owner") => Ok(()),
            _ => Err(AppError::GitUnauthorized)
        }
    } else {
        Err(AppError::GitUnauthorized)
    }
    
}