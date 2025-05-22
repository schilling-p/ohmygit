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
use infrastructure::diesel::DbPool;

#[debug_handler]
pub async fn login_user(session: Session, pool: State<DbPool>, Json(login_request): Json<LoginRequest>) -> Result<ApiResponse, AppError> {
    debug!("login request with: {:?} and {:?}", login_request.email, login_request.password);
    let user = retrieve_user_from_db(&pool, UserIdentifier::Email(login_request.email)).await?.0;
    verify_password(&login_request.password, &user.hashed_pw)?;

    session.insert("username", user.username.clone()).await?;
    let inserted_username: String = session.get("username").await?.unwrap_or("no username found".to_string());
    debug!("username cookie set to: {}", inserted_username);

    session.insert("user_email", user.email.clone()).await?;

    Ok(ApiResponse::Login(LoginResponse {
        message: "login_successful",
        user_email: user.email,
        username: user.username,
    }))
}