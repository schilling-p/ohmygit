use domain::models::{User, Repository};
use domain::request::repository::AuthorizationRequest;
use infrastructure::diesel::DbPool;
use crate::user::read::get_user_role_for_repository;
use error::AppError;
use uuid::Uuid;

pub async fn authorize_repository_action(pool: &DbPool, auth_request: AuthorizationRequest) -> Result<(), AppError> {
    let user_id: Uuid = auth_request.user.id;
    let repo_id: Uuid = auth_request.repo.id;
    match get_user_role_for_repository(pool, user_id, repo_id).await? {
        Some(role) => {
            if role != "admin" {
                return Err(AppError::GitUnauthorized);
            }
        },
        None => return Err(AppError::GitUnauthorized)
    }
    
    Ok(())    
}