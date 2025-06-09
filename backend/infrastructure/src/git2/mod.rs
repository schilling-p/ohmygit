use git2::{Repository, Error, Commit, Sort, BranchType, Oid};
use chrono::{DateTime};
use domain::response::repository::{CommitInformation, RepositoryFileInformation, RepositoryOverview};
use error::AppError;

pub struct GitRepository {
    pub repo: Repository,
}

impl GitRepository {
    pub fn open(path: &str) -> Result<GitRepository, AppError> {
        let repo: Repository = Repository::open(path)?;
        Ok(GitRepository { repo })
    }
    
    pub fn init_bare(path: &str) -> Result<(), AppError> {
        let repo = Repository::init_bare(path)?;
        Ok(())
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

    pub fn list_local_branches(&self) -> Result<Vec<String>, AppError> {
        let mut branches = Vec::new();
        for branch in self.repo.branches(Some(BranchType::Local))? {
            let (branch, _) = branch?;
            if let Some(name) = branch.name()? {
                branches.push(name.to_string());
            }
        }
        Ok(branches) 
    }

    pub fn create_branch(&self, new_branch: &str, base_branch: &str, switch_head: bool) -> Result<(), AppError> {
        let base = self.repo.find_branch(base_branch, BranchType::Local)?;
        let target = base.get().target().ok_or(Error::from_str("No target branch found"))?;
        let commit = self.repo.find_commit(target)?;
        self.repo.branch(new_branch, &commit, false)?;
        if switch_head {
            let reference = format!("refs/heads/{}", new_branch);
            self.repo.set_head(&reference)?;
        }

        Ok(())
    }

    pub fn get_repo_overview(&self, branch_name: Option<&str>) -> Result<RepositoryOverview, AppError> {
        let repo_name = self.get_repository_name()?;
        let head_commit = if let Some(branch_name) = branch_name {
            self.get_commit_from_branch(branch_name)?
        } else {
            self.get_head_commit()?
        };

        let tree = head_commit.tree()?;
        let head_commit_oid = head_commit.id();

        let mut files: Vec<RepositoryFileInformation> = Vec::new();
        for entry in tree.iter() {
            let file_name = entry.name().unwrap_or("").to_string();
            let (message, timestamp) = self.get_last_commit_from_path(&file_name, head_commit_oid)?;
            files.push(RepositoryFileInformation {
                file_name,
                last_commit_message: message,
                last_commit_time: timestamp,
            });
        }

        // TODO: insufficient error handling with unwrap()
        let head_commit_time = DateTime::from_timestamp(head_commit.time().seconds(), 0).unwrap();
        let commit_information = CommitInformation {
            commit_message: head_commit.message().unwrap_or("no commit yet").to_string(),
            commit_time: head_commit_time.to_string(),
        };

        let head_branch_name = match branch_name {
            Some(name) => name.to_owned(),
            None => self.get_branch_name_from_head()?,
        };

        Ok(RepositoryOverview {
            head_branch_name,
            repository_name: repo_name,
            latest_commit: commit_information,
            files,
        })
    }
}