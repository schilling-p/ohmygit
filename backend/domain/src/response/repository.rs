use serde::Serialize;
use crate::models::Repository;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct ListRepositoriesResponse {
    pub repositories: Vec<Repository>,
    pub user_email: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct RepositoryOverview {
    pub repository_name: String,
    pub head_branch_name: String,
    pub latest_commit: CommitInformation,
    pub files: Vec<RepositoryFileInformation>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct RepositoryFileInformation {
    pub file_name: String,
    pub last_commit_message: String,
    pub last_commit_time: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct CommitInformation {
    pub commit_message: String,
    pub commit_time: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct RepositoryBranches {
    pub branches: Vec<String>,
}