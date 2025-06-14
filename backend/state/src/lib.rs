use std::sync::Arc;
use application::authorization::service::AuthorizationService;
use domain::user::UserStore;
use domain::repository::store::RepositoryStore;
use domain::repository::git_store::GitRepositoryStore;

use infrastructure::diesel::connection::DbPool;
use infrastructure::diesel::user_store::DieselUserStore;
use infrastructure::diesel::repository_store::DieselRepositoryStore;

use application::user::service::UserService;
use application::repository::service::RepositoryService;
use domain::authorization::store::AuthorizationStore;
use domain::membership::store::MembershipStore;
use infrastructure::diesel::authorization_store::DieselAuthorizationStore;
use infrastructure::diesel::membership_store::DieselMembershipStore;
use infrastructure::git2::git_repository_store::Git2RepositoryStore;

#[derive(Clone)]
pub struct AppState {
    pub stores: Arc<AppStores>,
    pub services: Arc<AppServices>,
}

pub struct AppStores {
    pub users: Arc<dyn UserStore>,
    pub repos: Arc<dyn RepositoryStore>,
    pub auth: Arc<dyn AuthorizationStore>,
    pub members: Arc<dyn MembershipStore>,
    pub git_repos: Arc<dyn GitRepositoryStore>,
}

pub struct AppServices {
    pub user: Arc<UserService>,
    pub repo: Arc<RepositoryService>,
    pub auth: Arc<AuthorizationService>,
}
impl AppState {
    pub fn initialize_app_state(db: DbPool) -> Self {
        let users = Arc::new(DieselUserStore::new(db.clone()));
        let repos = Arc::new(DieselRepositoryStore::new(db.clone()));
        let auth = Arc::new(DieselAuthorizationStore::new(db.clone()));
        let members = Arc::new(DieselMembershipStore::new(db.clone(), users.clone()));
        let git_repos = Arc::new(Git2RepositoryStore);

        let user_service = Arc::new(UserService {user_store: users.clone(),});
        let auth_service = Arc::new(AuthorizationService { user_store: users.clone(), repo_store: repos.clone(), auth_store: auth.clone() });
        let repo_service = Arc::new(RepositoryService {user_store: users.clone(), repo_store: repos.clone(), git_store: git_repos.clone(), auth_service: auth_service.clone()});

        Self {
            stores: Arc::new(AppStores { users, repos, auth, members, git_repos }),
            services: Arc::new(AppServices { user: user_service, repo: repo_service, auth: auth_service}),
        }
    }
}