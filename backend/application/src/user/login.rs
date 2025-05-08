use axum::extract::State;
use axum_macros::debug_handler;
use axum::Json;
use super::read::find_user_by_email;
use shared::crypto::verify_password;
use error::AppError;
use tracing::debug;
use tower_sessions::Session;

use domain::request::auth::LoginRequest;
use domain::response::auth::LoginResponse;
use domain::ApiResponse;
use infrastructure::diesel::DbPool;

#[debug_handler]
pub async fn login_user(session: Session, pool: State<DbPool>, Json(login_request): Json<LoginRequest>) -> Result<ApiResponse, AppError> {
    debug!("LoginRequest: {:?}", &login_request);
    let user = find_user_by_email(&pool, &login_request.email).await?.0;
    verify_password(&login_request.password, &user.hashed_pw)?;
    session.insert("username", user.username.clone()).await?;
    debug!("username cookie set to: {}", user.username);
    session.insert("user_email", user.email.clone()).await?;
    debug!("user_email cookie set to: {}", user.email);

    Ok(ApiResponse::Login(LoginResponse {
        message: "login_successful",
        user_email: user.email,
        username: user.username,
    }))
}