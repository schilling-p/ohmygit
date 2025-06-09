use std::sync::Arc;
use domain::user::UserStore;

pub struct UserService {
    pub user_store: Arc<dyn UserStore>
}