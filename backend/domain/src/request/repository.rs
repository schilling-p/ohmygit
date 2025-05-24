use serde::{Deserialize, Serialize};
use crate::models::{User, Repository};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct InfoRefsQuery {
    pub service: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AuthorizationRequest {
    pub user: User,
    pub repo: Repository,
    pub action: RepoAction,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RepoAction {
    Clone,
    Push,
    OpenIssue,
    CommentOnIssue,
    CloseIssue,
    CreateMergeRequest,
    ApproveMergeRequest,
    ManageRepoSettings,
}