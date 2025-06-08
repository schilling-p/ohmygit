use async_trait::async_trait;
use error::AppError;
use crate::request::auth::{UserIdentifier};
use crate::user::model::{User, NewUser};

#[async_trait]
pub trait UserStore: Send + Sync {
    async fn list_users(&self) -> Result<Vec<User>, AppError>;
    async fn retrieve_user_by_identifier(&self, user_identifier: UserIdentifier) -> Result<User, AppError>;
    async fn write_user_to_db(&self, new_user: NewUser) -> Result<User, AppError>;
}