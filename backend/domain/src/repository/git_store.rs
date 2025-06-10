use async_trait::async_trait;
use git2::{Commit, Oid};
use error::AppError;
use crate::response::repository::RepositoryOverview;

#[async_trait]
pub trait GitRepositoryStore {
    async fn init_bare(path: &str) -> Result<(), AppError>;
    async fn get_head_commit(path: &str) -> Result<Commit, AppError>;
    async fn get_commit_from_branch(path: &str, branch_name: &str) -> Result<Commit, AppError>;
    async fn get_branch_name_from_head(path: &str) -> Result<String, AppError>;
    async fn list_tree_from_commit(commit: &Commit) -> Result<Vec<String>, AppError>;
    async fn get_repository_name(path: &str) -> Result<String, AppError>;
    async fn get_last_commit_from_path(path: &str, file_path: &str, from_oid: Oid) -> Result<(String, String), AppError>;
    async fn list_local_branches(path: &str) -> Result<Vec<String>, AppError>;
    async fn create_branch(path: &str, new_branch: &str, base_branch: &str, switch_head: bool) -> Result<(), AppError>;
    async fn get_repo_overview(path: &str, branch_name: Option<&str>) -> Result<RepositoryOverview, AppError>;
}