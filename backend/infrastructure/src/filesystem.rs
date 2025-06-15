use domain::filesystem::FileSystem;
use async_trait::async_trait;
use error::AppError;
use std::path::Path;
pub struct TokioFileSystem;

#[async_trait]
impl FileSystem for TokioFileSystem {
    async fn try_exists(&self, path: &Path) -> Result<bool, AppError> {
        Ok(tokio::fs::try_exists(path).await?)
    }

    async fn create_dir_all(&self, path: &Path) -> Result<(), AppError> {
        tokio::fs::create_dir_all(path).await?;
        Ok(())
    }
}
