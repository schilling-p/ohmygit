use std::sync::Arc;
use domain::user::UserStore;
use infrastructure::diesel::connection::DbPool;
use infrastructure::diesel::user_store::DieselUserStore;
use application::user::login::UserService;

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub stores: Arc<AppStores>,
}

pub struct AppStores {
    pub users: Arc<dyn UserStore>,
}

pub struct AppServices {
    pub user: Arc<UserService>
}

pub fn initialize_app_state(db: DbPool) -> AppState {
    let users = Arc::new(DieselUserStore::new(db.clone()));
    AppState {
        db,
        stores: Arc::new(AppStores { users }),
    }
}