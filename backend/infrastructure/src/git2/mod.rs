pub mod git_repository_store;

use git2::Repository;
use error::AppError;

pub struct GitRepository {
    pub repo: Repository,
}

impl GitRepository {
    pub fn open(path: &str) -> Result<GitRepository, AppError> {
        let repo: Repository = Repository::open(path)?;
        Ok(GitRepository { repo })
    }
}