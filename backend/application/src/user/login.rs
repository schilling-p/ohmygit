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
    // TODO: don't do clone() , requires new signature of find_user_by_email()
    match find_user_by_email(&pool, login_request.email.clone()).await {
        Ok(_) => Err(AppError::EmailAlreadyExists),
        Err(_) => {
            let user = find_user_by_email(&pool, login_request.email).await?;
            verify_password(&login_request.password, &user.hashed_pw)?;

            Ok(Json(LoginResponse { message: "Login successful".to_string() }))
        },
    }
}