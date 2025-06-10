use async_trait::async_trait;
use domain::repository::git_store::GitRepositoryStore;
use domain::response::repository::{RepositoryOverview, RepositoryFileInformation, CommitInformation};
use tokio::task::spawn_blocking;
use chrono::DateTime;
use error::AppError;
use git2::Repository as GitRepository;
use git2::{Commit, Error, Oid, Sort, BranchType};

pub struct Git2RepositoryStore;

#[async_trait]
impl GitRepositoryStore for Git2RepositoryStore {
    async fn init_bare(path: &str) -> Result<(), AppError> {
        let path = path.to_string();
        spawn_blocking(move || GitRepository::init_bare(path)).await??;
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
        let path = path.to_string();
        let from_oid = from_oid.clone();
        let result = spawn_blocking(move || -> Result<(String, String), AppError> {
            let repo = GitRepository::open(path)?;
            let mut revwalk = repo.revwalk()?;
            revwalk.push(from_oid)?;
            revwalk.set_sorting(Sort::TIME)?;

            for oid_result in revwalk {
                let oid = oid_result?;
                let commit = repo.find_commit(oid)?;

                if let Some(tree) = commit.tree().ok() {
                    if tree.get_path(std::path::Path::new(file_path)).is_ok() {
                        let msg = commit.summary().unwrap_or("").to_string();
                        let time = commit.time();
                        let ts = time.seconds();
                        let naive_datetime = DateTime::from_timestamp(ts, 0).unwrap();
                        return Ok((msg, naive_datetime.to_string()));
                    }
                }

            }
            Ok(("No commit".to_string(), "Unknown time".to_string()))
        }).await.map_err(AppError::from)??;

        Ok(result)
    }

    async fn list_local_branches(path: &str) -> Result<Vec<String>, AppError> {
        let path = path.to_string();
        let branches = spawn_blocking(move || -> Result<Vec<String>, AppError> {
            let mut branches = Vec::new();
            let repo = GitRepository::open(path)?;
            for branch in repo.branches(Some(BranchType::Local))? {
                let (branch, _) = branch?;
                if let Some(name) = branch.name()? {
                    branches.push(name.to_string());
                }

            }
            Ok(branches)
        }).await.map_err(AppError::from)??;
        Ok(branches)
    }

    async fn create_branch(path: &str, new_branch: &str, base_branch: &str, switch_head: bool) -> Result<(), AppError> {
        let path = path.to_string();
        let new_branch = new_branch.to_string();
        let base_branch = base_branch.to_string();
        spawn_blocking(move || -> Result<(), AppError> {
            let repo = GitRepository::open(path)?;
            let base = repo.find_branch(&base_branch, BranchType::Local)?;
            let target = base.get().target().ok_or(Error::from_str("No target branch found"))?;
            let commit = repo.find_commit(target)?;
            repo.branch(&new_branch, &commit, false)?;
            if switch_head {
                let reference = format!("refs/heads/{}", new_branch);
                repo.set_head(&reference)?;
            }
            Ok(())
        }).await.map_err(AppError::from)??;
        Ok(())
    }
    
    async fn get_repo_overview(path: &str, branch_name: Option<String>) -> Result<RepositoryOverview, AppError> {
        let path = path.to_string();
        let repo_name = Self::get_repository_name(&path).await?;
        let commit = match &branch_name {
            Some(branch) => Self::get_commit_from_branch(&path, branch).await?,
            None => Self::get_head_commit(&path).await?,
        };
        let head_commit_oid = commit.id();
        let files = {
            let entries = Self::list_tree_from_commit(&commit).await?;
            let mut result = Vec::new();
            for file_name in entries {
                let (message, timestamp) = Self::get_last_commit_from_path(&path, &file_name, head_commit_oid).await?;
                result.push(RepositoryFileInformation {
                    file_name, 
                    last_commit_message: message,
                    last_commit_time: timestamp,
                });
            }
            result
        };
        let commit_time = commit.time();
        let commit_dt = DateTime::from_timestamp(commit_time.seconds(), 0).unwrap();
        let commit_message = commit.summary().unwrap_or("no commit yet").to_string();
        
        let commit_info = CommitInformation {
            commit_message,
            commit_time: commit_dt.to_string(),
        };
        
        let head_branch_name = match branch_name {
            Some(name) => name,
            None => Self::get_branch_name_from_head(&path).await?,
        };
        
        Ok(RepositoryOverview {
            repository_name: repo_name,
            head_branch_name,
            latest_commit: commit_info,
            files,
        })        
    }

}
