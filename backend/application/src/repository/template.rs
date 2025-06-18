use domain::authorization::model::{AuthorizationRequest, RepoAction};
use domain::request::auth::UserIdentifier;
use super::service::RepositoryService;
use error::AppError;
use templating::RepositoryTemplate;

impl RepositoryService {
    pub async fn create_repository_view(&self, username: String, repository_name: String, branch_name: Option<String>, is_recently_authorized: bool) -> Result<RepositoryTemplate, AppError> {
        let repository = self.repo_store.retrieve_by_name(&repository_name).await?;
        if !repository.is_public && !is_recently_authorized {
            let repo_action = RepoAction::View;
            let user = self.user_store.retrieve_user_by_identifier(UserIdentifier::Username(username.clone())).await?;
            let auth_request = AuthorizationRequest {
                user_id: user.id,
                owner_id: repository.owner_id.unwrap(),
                repository_id: repository.id,
                repo_action,
            };
            
            self.auth_service.authorize_repository_action(auth_request).await?;
        }
        
        let repo_path = format!("/repos/{}/{}.git", username, repository_name);
        let repo_overview = self.git_store.as_ref().get_repo_overview(&repo_path, Option::from(&branch_name)).await?;
        
        let template = RepositoryTemplate {
            repository_name,
            username,
            overview: repo_overview,
        };
        
        Ok(template)
    }
}