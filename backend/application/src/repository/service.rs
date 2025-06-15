use std::sync::Arc;
use domain::filesystem::FileSystem;
use domain::repository::git_store::GitRepositoryStore;
use domain::repository::store::RepositoryStore;
use domain::user::UserStore;
use crate::authorization::service::AuthorizationService;

pub struct RepositoryService {
    pub repo_store: Arc<dyn RepositoryStore>,
    pub user_store: Arc<dyn UserStore>,
    pub git_store: Arc<dyn GitRepositoryStore>,
    pub auth_service: Arc<AuthorizationService>,
    pub file_system: Arc<dyn FileSystem>
}