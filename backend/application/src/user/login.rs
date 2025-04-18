use axum::extract::State;
use axum_macros::debug_handler;
use axum::Json;
use super::read::find_user_by_email;
use shared::crypto::verify_password;
use domain::models::LoginRequest;
use infrastructure::DbPool;
use error::AppError;
use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    message: String,
}

#[debug_handler]
pub async fn login_user(pool: State<DbPool>, Json(login_request): Json<LoginRequest>) -> Result<Json<LoginResponse>, AppError> {
    // TODO: clean up the clone()
    let login_request_password = login_request.password.clone();

    let user = find_user_by_email(&pool, login_request).await?;
    verify_password(&login_request_password, &user.hashed_pw).unwrap();

    Ok(Json(LoginResponse { message: "Login successful".to_string() }))
}
