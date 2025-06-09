use std::sync::Arc;
use domain::authorization::model::{AuthorizationRequest, Credentials, RepoAction, Role, UserRepoRole};
use domain::repository::store::RepositoryStore;
use domain::user::store::UserStore;
use domain::authorization::store::AuthorizationStore;
use domain::repository::model::Repository;
use domain::request::auth::{LoginRequest, UserIdentifier};
use error::AppError;

pub struct AuthorizationService {
    pub auth_store: Arc<dyn AuthorizationStore>,
    pub user_store: Arc<dyn UserStore>,
    repo_store: Arc<dyn RepositoryStore>
}

impl AuthorizationService {
    
    pub async fn authenticate_and_authorize_user(&self, credentials: Credentials, repository: Repository, repo_action: RepoAction) -> Result<(), AppError> {
        let login_request = LoginRequest {
            identifier: UserIdentifier::Username(credentials.username),
            password: credentials.password,
        };
        let user = self.user_store.retrieve_user_by_identifier(login_request.identifier).await?;
        let auth_request = AuthorizationRequest {
            user,
            repository,
            repo_action
        };
        self.authorize_repository_action(auth_request).await?;
        Ok(())
    }
    
    pub async fn authorize_repository_action(&self, auth_request: AuthorizationRequest) -> Result<(), AppError> {
        let user_id = auth_request.user.id;
        let repo_id = auth_request.repository.id;
        if auth_request.repository.owner_id == Some(user_id) {
            return Ok(());
        }

        let role_str = self.auth_store.get_user_role_for_repository(user_id, repo_id).await?;
        if let Some(role_str) = role_str {
            let role = Role::try_from(role_str)?;
            let user_repo_role = UserRepoRole { role };
            if user_repo_role.is_authorized_for(&auth_request.repo_action) {
                Ok(())
            } else {
                Err(AppError::Unauthorized)
            }

        } else {
            Err(AppError::Unauthorized)
        }
    }
}

