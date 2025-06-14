use async_trait::async_trait;
use error::AppError;
use crate::response::repository::RepositoryOverview;

#[async_trait]
pub trait GitRepositoryStore: Send + Sync {
    async fn init_bare(path: &str) -> Result<(), AppError>;
    async fn list_local_branches(path: &str) -> Result<Vec<String>, AppError>;
    async fn create_branch(path: &str, new_branch: &str, base_branch: &str, switch_head: bool) -> Result<(), AppError>;
    async fn get_repo_overview(path: &str, branch_name: Option<String>) -> Result<RepositoryOverview, AppError>;
}