use domain::repository::model::Repository;
use domain::request::auth::UserIdentifier;
use super::service::RepositoryService;
use error::AppError;

impl RepositoryService {
    pub async fn list_user_repositories(&self, username: String) -> Result<Vec<Repository>, AppError> {
        let user = self.user_store.retrieve_user_by_identifier(UserIdentifier::Email(username)).await?;
        let repos = self.repo_store.list_user_repositories(user.id).await?;
        Ok(repos)
    }
}