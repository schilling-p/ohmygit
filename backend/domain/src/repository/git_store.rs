use error::AppError;
use crate::response::repository::RepositoryOverview;
use std::pin::Pin;

pub trait GitRepositoryStore: Send + Sync {
    fn init_bare<'a>(&'a self, path: &'a str) -> Pin<Box<dyn Future<Output = Result<(), AppError>> + Send + 'a>>;
    fn list_local_branches<'a>(&'a self, path: &'a str) -> Pin<Box<dyn Future<Output = Result<Vec<String>, AppError>> + Send + 'a>>;
    fn create_branch<'a>(&'a self, path: &'a str, new_branch: &'a str, base_branch: &'a str, switch_head: bool) -> Pin<Box<dyn Future<Output = Result<(), AppError>> + Send + 'a>>;
    fn get_repo_overview<'a>(&'a self, path: &'a str, branch_name: Option<&'a String>) -> Pin<Box<dyn Future<Output = Result<RepositoryOverview, AppError>> + Send + 'a>>;
}