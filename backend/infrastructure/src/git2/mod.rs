use git2::{Repository, Error, Commit, Tree, Oid};
use error::AppError;

pub struct GitRepository {
    pub repo: Repository,
}

impl GitRepository {
    pub fn open(path: &str) -> Result<GitRepository, AppError> {
        let repo = Repository::open(path)?;
        Ok(GitRepository { repo })
    }
    
    pub fn get_head_commit(&self) -> Result<Commit, Error> {
        let head = self.repo.head()?;
        let commit = head.peel_to_commit()?;
        Ok(commit)
    }
    
    pub fn list_tree(&self, commit: &Commit) -> Result<Vec<String>, Error> {
        let tree = commit.tree()?;
        let mut entries = Vec::new();
        for entry in tree.iter() {
            let name = entry.name().unwrap_or("").to_string();
            entries.push(name);
        }
        Ok(entries)        
    }
    
    pub fn get_repository_name(&self) -> Result<String, AppError> {
        Ok(self.repo.head()?.name().unwrap_or("").to_string())
    }
}