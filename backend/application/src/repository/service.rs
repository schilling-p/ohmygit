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
    
    pub async fn create_new_user_repository(&self, username: String, create_repo_request: CreateRepoRequest) -> Result<(), AppError> {
        let user = self.user_store.retrieve_user_by_identifier(UserIdentifier::Username(username)).await?;
        let new_user_repository = NewUserRepository {
            owner_id : user.id,
            name: create_repo_request.repository_name,
            is_public: create_repo_request.is_public,
            description: create_repo_request.description,
        };
        
        Ok(self.repo_store.write_repo_to_db(new_user_repository).await?)
    }
}