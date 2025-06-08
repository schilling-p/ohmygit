use async_trait::async_trait;
use error::AppError;
use crate::repository::model::Repository;

#[async_trait]
pub trait RepositoryStore: Send +Sync {
    async fn find_repository_by_name(&self, repo_name: &str) -> Result<Repository, AppError>;
    
}