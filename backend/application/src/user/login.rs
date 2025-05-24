use axum::extract::State;
use axum_macros::debug_handler;
use axum::Json;
use super::read::retrieve_user_from_db;
use shared::crypto::verify_password;
use error::AppError;
use tracing::debug;
use tower_sessions::{Session};

use domain::request::auth::{LoginRequest, UserIdentifier};
use domain::response::auth::LoginResponse;
use domain::ApiResponse;
use domain::models::User;
use infrastructure::diesel::DbPool;

#[debug_handler]
pub async fn user_login_handler(session: Session, pool: State<DbPool>, Json(login_request): Json<LoginRequest>) -> Result<ApiResponse, AppError> {

    match login_user(pool, login_request).await {
        Ok(user) => {
            session.insert("username", user.username.clone()).await?;
            session.insert("user_email", user.email.clone()).await?;

            Ok(ApiResponse::Login(LoginResponse {
                message: "login_successful",
                // TODO: remove for production, is not needed anymore
                user_email: user.email.clone(),
                username: user.username.clone(),
            }))
        }
        Err(e) => Err(e),
    }
}

pub async fn login_user(pool: State<DbPool>, login_request: LoginRequest) -> Result<User, AppError> {
    let user = retrieve_user_from_db(&pool, login_request.identifier).await?;
    match verify_password(&login_request.password, &user.hashed_pw) {
        Ok(_) => Ok(user),
        Err(_) => Err(AppError::Unauthorized),
    }
}