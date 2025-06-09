use std::sync::Arc;
use domain::user::UserStore;
use domain::repository::store::RepositoryStore;

use infrastructure::diesel::connection::DbPool;
use infrastructure::diesel::user_store::DieselUserStore;
use infrastructure::diesel::repository_store::DieselRepositoryStore;

use application::user::service::UserService;
use application::repository::service::RepositoryService;

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub stores: Arc<AppStores>,
    pub services: Arc<AppServices>,
}

pub struct AppStores {
    pub users: Arc<dyn UserStore>,
    pub repos: Arc<dyn RepositoryStore>,
}

pub struct AppServices {
    pub user: Arc<UserService>,
    pub repo: Arc<RepositoryService>
}

pub fn initialize_app_state(db: DbPool) -> AppState {
    let users = Arc::new(DieselUserStore::new(db.clone()));
    let repos = Arc::new(DieselRepositoryStore::new(db.clone()));
    let user_service = Arc::new(UserService {store: users.clone(),});
    let repo_service = Arc::new(RepositoryService {user_store: users.clone(), repo_store: repos.clone()})
    AppState {
        db,
        stores: Arc::new(AppStores { users, repos }),
        services: Arc::new(AppServices { user: user_service, repo: repo_service }),
    }
}