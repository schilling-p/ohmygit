use std::sync::Arc;
use domain::repository::store::RepositoryStore;
use domain::repository::model::{NewUserRepository, Repository};
use domain::user::{User, UserStore};
use domain::request::auth::UserIdentifier;
use domain::request::repository::CreateRepoRequest;
use templating::RepositoryTemplate;
use error::AppError;

pub struct RepositoryService {
    pub repo_store: Arc<dyn RepositoryStore>,
    pub user_store: Arc<dyn UserStore>
}

impl RepositoryService {
    pub async fn build_repository_view(&self, user: User, repo_name: String, branch: Option<String>) -> Result<RepositoryTemplate, AppError> {
        !unimplemented!()
    }
}