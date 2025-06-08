use axum::extract::State;
use axum_macros::debug_handler;
use axum::Json;
use domain::request::auth::UserIdentifier;
use domain::user::{NewUser, User};
use shared::state::AppState;
use shared::crypto::hash_password;
use error::AppError;

#[debug_handler]
pub async fn list_users(State(app_state): State<AppState>) -> Result<Json<Vec<User>>, AppError> {
    let users = app_state.stores.users.list_users().await?;
    Ok(Json(users))
}

#[debug_handler]
pub async fn user_sign_up(State(app_state): State<AppState>, Json(mut new_user): Json<NewUser>) -> Result<Json<User>, AppError> {
    let username = new_user.username.clone();
    match app_state.stores.users.retrieve_user_by_identifier(UserIdentifier::Username(username)).await? {
        Ok(_) => return Err(AppError::EmailAlreadyExists),
        Err(AppError::NotFound(_)) => {},
        Err(e) => return Err{ 0: e },
    }

    new_user.hashed_pw = hash_password(&new_user.hashed_pw)?;
    let user = app_state.stores.users.write_user_to_db(new_user).await?;
    Ok(Json(user))    
}