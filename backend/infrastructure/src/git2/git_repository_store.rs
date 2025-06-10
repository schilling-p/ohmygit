use async_trait::async_trait;
use domain::repository::git_store::GitRepositoryStore;
use domain::response::repository::RepositoryOverview;
use tokio::task::spawn_blocking;
use error::AppError;
use git2::{BranchType, Repository as GitRepository};
use git2::{Commit, Error, Oid};

pub struct Git2RepositoryStore;

#[async_trait]
impl GitRepositoryStore for Git2RepositoryStore {
    async fn init_bare(path: &str) -> Result<(), AppError> {
        let path = path.to_string();
        let repo = spawn_blocking(move || GitRepository::init_bare(path)).await??;
        Ok(())
    }

    async fn get_head_commit(path: &str) -> Result<Commit, AppError> {
        let path = path.to_string();
        let commit = spawn_blocking(move || -> Result<Commit, AppError> {
            let repo = GitRepository::open(path)?;
            let head = repo.head()?;
            let commit = head.peel_to_commit()?;
            Ok(commit)
        }).await.map_err(AppError::from)??;

        Ok(commit)
    }

    async fn get_commit_from_branch(path: &str, branch_name: &str) -> Result<Commit, AppError> {
        let path = path.to_string();
        let commit = spawn_blocking(move || -> Result<Commit, AppError> {
            let repo = GitRepository::open(path)?;
            let branch = repo.find_branch(branch_name, BranchType::Local)?;
            let target = branch.get() .target().ok_or(Error::from_str("No target branch found"))?;
            let commit = repo.find_commit(target)?;
            Ok(commit)
        }).await.map_err(AppError::from)??;
        Ok(commit)
    }

    async fn get_branch_name_from_head(path: &str) -> Result<String, AppError> {
        let path = path.to_string();
        let branch_name = spawn_blocking(move || -> Result<String, AppError> {
            let repo = GitRepository::open(path)?;
            let head = repo.head()?;
            let branch_name = head.shorthand().unwrap_or("").to_string();
            Ok(branch_name)
        }).await.map_err(AppError::from)??;
        Ok(branch_name)
    }

    async fn list_tree_from_commit(commit: &Commit) -> Result<Vec<String>, AppError> {
        let entries = spawn_blocking(move || -> Result<Vec<String>, AppError> {
            let tree = commit.tree()?;
            let mut entries = Vec::new();
            for entry in tree.iter() {
                let name: String = entry.name().unwrap_or("").to_string();
                entries.push(name);
            }
            Ok(entries)
        }).await.map_err(AppError::from)??;
        Ok(entries)
    }
    
    async fn get_repository_name(path: &str) -> Result<String, AppError> {
        let path = path.to_string();
        let repo_name = spawn_blocking(move || -> Result<String, AppError> {
            let repo = GitRepository::open(path)?;
            let repo_name = repo.head()?.name().unwrap_or("No Head found").to_string();
            Ok(repo_name)
        }).await.map_err(AppError::from)??;
        Ok(repo_name)
    }
    
    async fn get_last_commit_from_path(path: &str, file_path: &str, from_oid: Oid) -> Result<(String, String), AppError> {
        
    }
}
