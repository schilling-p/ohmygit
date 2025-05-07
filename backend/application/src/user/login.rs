use axum::extract::State;
use axum_macros::debug_handler;
use axum::Json;
use super::read::find_user_by_email;
use shared::crypto::verify_password;
use error::AppError;
use tracing::debug;

use domain::request::auth::LoginRequest;
use domain::response::auth::LoginResponse;
use domain::ApiResponse;
use infrastructure::diesel::DbPool;

#[debug_handler]
pub async fn login_user(pool: State<DbPool>, Json(login_request): Json<LoginRequest>) -> Result<ApiResponse, AppError> {
    debug!("LoginRequest: {:?}", &login_request);
    let user = find_user_by_email(&pool, &login_request.email).await?.0;
    verify_password(&login_request.password, &user.hashed_pw)?;

    Ok(ApiResponse::Login(LoginResponse {
        message: "login_successful",
        user_email: user.email,
        username: user.username,
    }))
}