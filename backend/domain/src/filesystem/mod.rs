use async_trait::async_trait;
use error::AppError;
use std::path::Path;

#[async_trait]
pub trait FileSystem: Send + Sync {
    async fn try_exists(&self, path: &Path) -> Result<bool, AppError>;
    async fn create_dir_all(&self, path: &Path) -> Result<(), AppError>;
}