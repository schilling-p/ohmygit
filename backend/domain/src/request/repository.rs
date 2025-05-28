use serde::{Deserialize, Serialize};
use crate::models::{User, Repository};
use std::convert::TryFrom;
use error::AppError;

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

impl TryFrom<&str> for RepoAction {
    type Error = AppError;
    
    fn try_from(service: &str) -> Result<Self, Self::Error> {
        match service {
            "git-upload-pack" => Ok(RepoAction::Clone),
            "git-receive-pack" => Ok(RepoAction::Push),
            _ => Err(AppError::BadRequest(format!("Unknown Git service: {}", service)))
        }
    }    
}