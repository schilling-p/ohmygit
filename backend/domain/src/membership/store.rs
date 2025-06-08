use async_trait::async_trait;
use uuid::Uuid;
use error::AppError;
use crate::organization::model::Organization;

#[async_trait]
pub trait MembershipStore: Send + Sync {
    async fn get_organizations_for_user(&self, user_id: Uuid) -> Result<Vec<Organization>, AppError>;
}