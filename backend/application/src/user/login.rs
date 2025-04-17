use axum::extract::State;
use super::read::find_user_by_email;
use shared::crypto::verify_password;
use domain::models::LoginRequest;
use axum::http::StatusCode;
use infrastructure::DbPool;
use axum_macros::debug_handler;
use axum::Json;

#[debug_handler]
pub async fn login_user(pool: State<DbPool>, Json(login_request): Json<LoginRequest>) -> Result<(), (StatusCode, String)> {
    let user = find_user_by_email(&pool, login_request).await?;
    verify_password(&user.hashed_pw, &user.hashed_pw).unwrap();
    Ok(())
}
