use domain::request::auth::UserIdentifier;
use domain::user::{NewUser, User};
use error::AppError;
use shared::crypto::{hash_password};
use crate::user::service::UserService;

impl UserService {
    pub async fn user_signup(&self, mut new_user: NewUser) -> Result<User, AppError> {
        match self.user_store.retrieve_user_by_identifier(UserIdentifier::Username(new_user.username.clone())).await {
            Ok(_) => return Err(AppError::UserAlreadyExists),
            Err(AppError::NotFound(_)) => {
                match self.user_store.retrieve_user_by_identifier(UserIdentifier::Email(new_user.email.clone())).await {
                    Ok(_) => return Err(AppError::UserAlreadyExists),
                    Err(_) => {}
                }
            },
            Err(e) => return Err{ 0: e },
        }

        new_user.hashed_pw = hash_password(&new_user.hashed_pw)?;
        let user = self.user_store.write_user_to_db(new_user).await?;
        Ok(user)
    }
}