use axum_extra::{headers, typed_header::TypedHeader};
use axum_extra::headers::authorization::Basic;
use headers::Authorization;
use serde::{Deserialize, Serialize};
use crate::models::{User, Repository};
use std::convert::TryFrom;
use error::AppError;
use crate::request::auth::UserIdentifier;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct InfoRefsQuery {
    pub service: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct CreateBranchRequest {
    pub new_branch_name: String,
    pub base_branch_name: String,
    pub switch_head: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AuthorizationRequest {
    pub user: User,
    pub repository: Repository,
    pub repo_action: RepoAction,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RepoAction {
    View,
    Clone,
    Push,
    OpenIssue,
    CommentOnIssue,
    CloseIssue,
    CreateMergeRequest,
    ApproveMergeRequest,
    ManageRepoSettings,
    CreateBranch,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct CreateRepoRequest {
    pub repository_name: String,
    pub description: Option<String>,
    pub is_private: bool,
}

impl From<TypedHeader<Authorization<Basic>>> for Credentials {
    fn from(auth: TypedHeader<Authorization<Basic>>) -> Self {
        Credentials {
            username: auth.username().to_string(),
            password: auth.password().to_string(),
        }
    }
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