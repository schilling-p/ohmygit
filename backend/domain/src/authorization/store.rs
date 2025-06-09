use async_trait::async_trait;
use uuid::Uuid;
use error::AppError;

#[async_trait]
pub trait AuthorizationStore: Send + Sync {
    async fn get_user_role_for_repository(&self, id_user: Uuid, repo_id: Uuid) -> Result<Option<String>, AppError>;
}