use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Basic;
use axum_extra::TypedHeader;
use chrono::{DateTime, Utc};
use diesel::{Associations, Identifiable, QueryId, Queryable, QueryableByName, Selectable};
use serde::Serialize;
use uuid::Uuid;

use error::AppError;
use crate::authorization::model::Role::{Developer, Guest, Maintainer, Owner};
use crate::repository::model::Repository;
use crate::user::model::User;
use crate::schema::user_repository_roles;

#[derive(Selectable, Queryable, Identifiable, QueryableByName, Associations, Serialize, QueryId, Clone, Debug, PartialEq)]
#[diesel(table_name = user_repository_roles)]
#[diesel(belongs_to(Repository, foreign_key = repository_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(primary_key(user_id, repository_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRepositoryRoles {
    pub user_id: Uuid,
    pub repository_id: Uuid,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Debug, PartialEq, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AuthorizationRequest {
    pub user_id: Uuid,
    pub owner_id: Uuid,
    pub repository_id: Uuid,
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

#[derive(Debug, PartialEq, Clone)]
pub struct UserRepoRole {
    pub role: Role,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Role {
    Guest,
    Developer,
    Maintainer,
    Owner,
    Admin,
}

impl UserRepoRole {
    pub fn is_authorized_for(&self, action: &RepoAction) -> bool {
        use RepoAction::*;
        use Role::*;
        match (action, &self.role) {
            (View, Guest | Developer | Maintainer | Owner) => true,
            (Clone, Guest | Developer | Maintainer | Owner) => true,
            (Push, Developer | Maintainer | Owner) => true,
            (OpenIssue, Developer | Maintainer | Owner) => true,
            (CommentOnIssue, Developer | Maintainer | Owner) => true,
            (CloseIssue, Developer | Maintainer | Owner) => true,
            (CreateMergeRequest, Developer | Maintainer | Owner) => true,
            (ApproveMergeRequest, Developer | Maintainer | Owner) => true,
            (ManageRepoSettings, Maintainer | Owner) => true,
            (CreateBranch, Developer | Maintainer |Owner) => true,
            _ => false,
        }
    }
}

impl From<TypedHeader<Authorization<Basic>>> for Credentials {
    fn from(auth: TypedHeader<Authorization<Basic>>) -> Self {
        Credentials {
            username: auth.username().to_string(),
            password: auth.password().to_string(),
        }
    }
}

impl TryFrom<String> for Role {
    type Error = AppError;
    fn try_from(role: String) -> Result<Self, Self::Error> {
        match role.as_str() {
            "guest" => Ok(Guest),
            "developer" => Ok(Developer),
            "maintainer" => Ok(Maintainer),
            "owner" => Ok(Owner),
            "admin" => Ok(Role::Admin),
            _ => Err(AppError::BadRequest(format!("Unknown role: {}", role)))
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