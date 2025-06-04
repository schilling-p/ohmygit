use axum::extract::State;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Basic;
use axum_extra::TypedHeader;
use domain::models::Repository;
use domain::request::repository::{AuthorizationRequest, Credentials, RepoAction};
use infrastructure::diesel::DbPool;
use crate::user::read::{get_user_role_for_repository, retrieve_user_from_db};
use error::AppError;
use uuid::{Uuid};
use tracing::debug;
use domain::request::auth::{LoginRequest, UserIdentifier};
use crate::repository::read::find_repository_by_name;
use crate::user::login::login_user;

pub async fn authenticate_and_authorize_user(pool: &State<DbPool>, auth_header: TypedHeader<Authorization<Basic>>, repository: Repository, repo_action: RepoAction) -> Result<(), AppError> {
    let credentials = Credentials::from(auth_header);
    let login_request = LoginRequest {
        identifier: UserIdentifier::Username(credentials.username),
        password: credentials.password,
    };
    let user = login_user(&pool, login_request).await.map_err(|_| AppError::GitUnauthorized("Credentials don't check out.".to_string()))?;

    let auth_request = AuthorizationRequest {
        user, repository, repo_action,
    };
    authorize_repository_action(&pool, auth_request).await?;

    Ok(())
}

pub async fn create_authorization_request(pool: &DbPool, username: String, repo_name: String, repo_action: RepoAction) -> Result<AuthorizationRequest, AppError> {
    let repository = find_repository_by_name(&pool, &repo_name).await?;
    let user = retrieve_user_from_db(&pool, UserIdentifier::Username(username)).await?;
    Ok(AuthorizationRequest {
        user, repository, repo_action,
    })   
}


pub async fn authorize_repository_action(pool: &DbPool, auth_request: AuthorizationRequest) -> Result<(), AppError> {
    let user_id: Uuid = auth_request.user.id;
    let repo_id: Uuid = auth_request.repository.id;
    if let Some(owner_id) = auth_request.repository.owner_id {
        if owner_id == user_id {
            return Ok(())
        }
    }
    
    let role = get_user_role_for_repository(pool, user_id, repo_id).await?;
    let debug_role = role.clone();
    debug!("User role for repository: {:?}", debug_role.unwrap_or("nothing found".to_string()));
    if let Some(role) = role {
        match (auth_request.repo_action, role.as_str()) {
            (RepoAction::View, "reader" | "developer" | "maintainer" | "owner") => Ok(()),
            (RepoAction::Clone, "reader" | "developer" | "maintainer" | "owner") => Ok(()),
            (RepoAction::Push, "developer" | "maintainer" | "owner") => Ok(()),
            (RepoAction::OpenIssue, "developer" | "maintainer" | "owner") => Ok(()),
            (RepoAction::CommentOnIssue, "developer" | "maintainer" | "owner") => Ok(()),
            (RepoAction::CloseIssue, "developer" | "maintainer" | "owner") => Ok(()),
            (RepoAction::CreateMergeRequest, "developer" | "maintainer" | "owner") => Ok(()),
            (RepoAction::ApproveMergeRequest, "developer" | "maintainer" | "owner") => Ok(()),
            (RepoAction::ManageRepoSettings, "maintainer" | "owner") => Ok(()),
            (RepoAction::CreateBranch, "developer" | "maintainer" | "owner") => Ok(()),
            _ => Err(AppError::GitUnauthorized("User role does not have permission to perform this action.".to_string()))
        }
    } else {
        Err(AppError::GitUnauthorized("Could not find any user role for repository and user.".to_string()))
    }    
}