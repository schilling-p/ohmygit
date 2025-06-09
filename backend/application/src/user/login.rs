use shared::crypto::verify_password;
use error::AppError;

use domain::request::auth::{LoginRequest};
use domain::user::User;
use super::service::UserService;

impl UserService {
    pub async fn user_login(&self, login_request: LoginRequest) -> Result<User, AppError> {
        let user = self.user_store.retrieve_user_by_identifier(login_request.identifier).await?;
        verify_password(&login_request.password, &user.hashed_pw)?;
        Ok(user)
    }
}