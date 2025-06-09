use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct InfoRefsQuery {
    pub service: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct CreateBranchRequest {
    pub new_branch_name: String,
    pub base_branch_name: String,
    pub switch_head: bool,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct CreateRepoRequest {
    pub repository_name: String,
    pub description: Option<String>,
    pub is_public: bool,
}