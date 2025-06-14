pub mod git_repository_store;
use chrono::DateTime;
use git2::{Repository, Commit, Error, BranchType, Oid, Sort};
use error::AppError;

pub struct GitRepository {
    pub repo: Repository,
}

impl GitRepository {
    pub fn open(path: &str) -> Result<GitRepository, AppError> {
        let repo: Repository = Repository::open(path)?;
        Ok(GitRepository { repo })
    }

    pub fn get_head_commit(&self) -> Result<Commit, AppError> {
        let head = self.repo.head()?;
        let commit = head.peel_to_commit()?;
        Ok(commit)
    }

    pub fn get_commit_from_branch(&self, branch_name: &str) -> Result<Commit, AppError> {
        let branch = self.repo.find_branch(branch_name, BranchType::Local)?;
        let target = branch.get().target().ok_or(Error::from_str("No target branch found"))?;
        let commit = self.repo.find_commit(target)?;
        Ok(commit)
    }

    pub fn get_branch_name_from_head(&self) -> Result<String, AppError> {
        let head = self.repo.head()?;
        let head_branch = head.shorthand().unwrap_or("").to_string();
        Ok(head_branch)
    }

    pub fn list_tree_from_commit(&self, commit: &Commit) -> Result<Vec<String>, AppError> {
        let tree = commit.tree()?;
        let mut entries: Vec<String> = Vec::new();
        for entry in tree.iter() {
            let name: String = entry.name().unwrap_or("").to_string();
            entries.push(name);
        }
        Ok(entries)
    }

    pub fn get_repository_name(&self) -> Result<String, AppError> {
        Ok(self.repo.head()?.name().unwrap_or("No Head found").to_string())
    }

    pub fn get_last_commit_from_path(&self, file_path: &str, from_oid: Oid) -> Result<(String, String), AppError> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push(from_oid)?;
        revwalk.set_sorting(Sort::TIME)?;

        for oid_result in revwalk {
            let oid = oid_result?;
            let commit = self.repo.find_commit(oid)?;

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

        Ok(("No commit".into(), "Unknown time".into()))
    }
}