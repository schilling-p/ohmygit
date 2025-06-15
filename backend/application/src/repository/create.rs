use domain::repository::model::NewUserRepository;
use domain::request::auth::UserIdentifier;
use domain::request::repository::CreateRepoRequest;
use super::service::RepositoryService;
use error::AppError;

impl RepositoryService {
    pub async fn create_new_user_repository(&self, username: String, create_repo_request: CreateRepoRequest) -> Result<(), AppError> {
        let user = self.user_store.retrieve_user_by_identifier(UserIdentifier::Username(username)).await?;
        let repo_name = create_repo_request.repository_name.clone();
        match self.repo_store.retrieve_by_owner_and_name(user.id, &repo_name).await {
            Ok(_) => return Err(AppError::RepositoryAlreadyExists),
            Err(AppError::NotFound(_)) => {},
            Err(e) => return Err{ 0: e },
        }

        let new_user_repository = NewUserRepository {
            owner_id : user.id,
            name: create_repo_request.repository_name,
            is_public: create_repo_request.is_public,
            description: create_repo_request.description,
        };

        self.repo_store.write_repo_to_db(new_user_repository).await?;
        Ok(())
    }
}