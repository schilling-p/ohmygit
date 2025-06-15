use std::pin::Pin;
use domain::repository::git_store::GitRepositoryStore;
use domain::response::repository::{RepositoryOverview, RepositoryFileInformation, CommitInformation};
use tokio::task::spawn_blocking;
use chrono::DateTime;
use error::AppError;
use crate::git2::GitRepository;
use git2::{Error, BranchType};

pub struct Git2RepositoryStore;

impl GitRepositoryStore for Git2RepositoryStore {
    fn init_bare<'a>(&'a self, path: &'a str) -> Pin<Box<dyn Future<Output = Result<(), AppError>> + Send + 'static>> {
        let path = path.to_string();
        Box::pin(async move {
            spawn_blocking(move || {
                git2::Repository::init_bare(path)?;
                Ok(())
            }).await.map_err(AppError::from)?
        })
    }

    fn list_local_branches<'a>(&'a self, path: &'a str) -> Pin<Box<dyn Future<Output = Result<Vec<String>, AppError>> + Send + 'static>> {
        let path = path.to_string();
        Box::pin(async move {
            let branches = spawn_blocking(move || -> Result<Vec<String>, AppError> {
                let mut branches = Vec::new();
                let repo = GitRepository::open(&path)?;
                for branch in repo.repo.branches(Some(BranchType::Local))? {
                    let (branch, _) = branch?;
                    if let Some(name) = branch.name()? {
                        branches.push(name.to_string());
                    }
                }
                Ok(branches)
            }).await.map_err(AppError::from)??;
            Ok(branches)
        })        
    }

    fn create_branch<'a>(&'a self, path: &'a str, new_branch: &'a str, base_branch: &'a str, switch_head: bool) -> Pin<Box<dyn Future<Output = Result<(), AppError>> + Send + 'static>> {
        let path = path.to_string();
        let new_branch = new_branch.to_string();
        let base_branch = base_branch.to_string();
        Box::pin(async move {
            spawn_blocking(move || -> Result<(), AppError> {
                let repo = GitRepository::open(&path)?;
                let base = repo.repo.find_branch(&base_branch, BranchType::Local)?;
                let target = base.get().target().ok_or(Error::from_str("No target branch found"))?;
                let commit = repo.repo.find_commit(target)?;
                repo.repo.branch(&new_branch, &commit, false)?;
                if switch_head {
                    let reference = format!("refs/heads/{}", new_branch);
                    repo.repo.set_head(&reference)?;
                }
                Ok(())
            }).await.map_err(AppError::from)??;
            Ok(())
        })
    }

    fn get_repo_overview<'a>(&'a self, path: &'a str, branch_name: Option<&'a String>) -> Pin<Box<dyn Future<Output = Result<RepositoryOverview, AppError>> + Send + 'static>> {
        let path = path.to_string();
        let branch_name = branch_name.map(|name| name.to_owned());

        Box::pin(async move {
            let repo_overview = spawn_blocking(move || -> Result<RepositoryOverview, AppError> {
                let repo = GitRepository::open(&path)?;
                let commit = match &branch_name {
                    Some(branch) => repo.get_commit_from_branch(branch)?,
                    None => repo.get_head_commit()?,
                };
                let head_commit_oid = commit.id();

                let entries = repo.list_tree_from_commit(&commit)?;
                let mut files = Vec::new();
                for file_name in entries {
                    let (message, timestamp) = repo.get_last_commit_from_path(&file_name, head_commit_oid)?;
                    files.push(RepositoryFileInformation {
                        file_name,
                        last_commit_message: message,
                        last_commit_time: timestamp,
                    });
                }

                let commit_time = commit.time();
                let commit_dt = DateTime::from_timestamp(commit_time.seconds(), 0).unwrap();
                let commit_message = commit.summary().unwrap_or("no commit yet").to_string();

                let commit_info = CommitInformation {
                    commit_message,
                    commit_time: commit_dt.to_string(),
                };

                let head_branch_name = match branch_name {
                    Some(name) => name,
                    None => repo.get_branch_name_from_head()?,
                };

                let repo_name = repo.get_repository_name()?;

                Ok(RepositoryOverview {
                    repository_name: repo_name,
                    head_branch_name,
                    latest_commit: commit_info,
                    files,
                })
            }).await.map_err(AppError::from)??;

            Ok(repo_overview)
        })
    }
}
