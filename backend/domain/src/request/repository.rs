use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct FetchRepositoriesRequest {
    pub user_email: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct RepositoryPath {
    pub username: String,
    pub repository_name: String,
}