use std::sync::Arc;
use domain::user::UserStore;
use infrastructure::diesel::connection::DbPool;
use infrastructure::diesel::user_store::DieselUserStore;
use application::user::service::UserService;

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub stores: Arc<AppStores>,
    pub services: Arc<AppServices>,
}

pub struct AppStores {
    pub users: Arc<dyn UserStore>,
}

pub struct AppServices {
    pub user: Arc<UserService>
}

pub fn initialize_app_state(db: DbPool) -> AppState {
    let users = Arc::new(DieselUserStore::new(db.clone()));
    let user_service = Arc::new(UserService {store: users.clone(),});
    AppState {
        db,
        stores: Arc::new(AppStores { users }),
        services: Arc::new(AppServices { user: user_service }),
    }
}