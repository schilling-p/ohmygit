use async_trait::async_trait;
use uuid::Uuid;
use error::AppError;
use crate::repository::model::{NewRepositoryBranch, NewUserRepository, Repository};

#[async_trait]
pub trait RepositoryStore: Send +Sync {
    async fn retrieve_by_name(&self, repo_name: &str) -> Result<Repository, AppError>;
    async fn retrieve_by_owner_and_name(&self, owner_id: Uuid, repo_name: &str) -> Result<Repository, AppError>;
    async fn list_user_repositories(&self, user_id: Uuid) -> Result<Vec<Repository>, AppError>;
    async fn write_repo_to_db(&self, new_repo: NewUserRepository) -> Result<(), AppError>;
    async fn write_repo_branch_to_db(&self, new_branch: NewRepositoryBranch) -> Result<(), AppError>;
}