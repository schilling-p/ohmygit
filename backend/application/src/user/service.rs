use std::sync::Arc;
use domain::user::UserStore;

pub struct UserService {
    pub store: Arc<dyn UserStore>
}