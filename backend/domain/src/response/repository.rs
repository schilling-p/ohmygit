use serde::Serialize;
use crate::models::Repository;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct ListRepositoriesResponse {
    pub repositories: Vec<Repository>,
    pub user_email: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct RepositoryOverview {
    pub name: String,
    pub latest_commit: String,
    pub files: Vec<String>,
}