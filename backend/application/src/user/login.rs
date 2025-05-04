use axum::extract::State;
use axum_macros::debug_handler;
use axum::Json;
use super::read::find_user_by_email;
use shared::crypto::verify_password;
use domain::request::auth::LoginRequest;
use domain::response::auth::LoginResponse;
use infrastructure::DbPool;
use error::AppError;
use tracing::debug;

#[debug_handler]
pub async fn login_user(pool: State<DbPool>, Json(login_request): Json<LoginRequest>) -> Result<Json<LoginResponse>, AppError> {
    // TODO: don't do clone() , solving this requires a new signature of find_user_by_email()
    debug!("LoginRequest: {:?}", &login_request);
    let user = find_user_by_email(&pool, &login_request.email).await?.0;
    verify_password(&login_request.password, &user.hashed_pw)?;

    debug!("login successful");
    Ok(Json(LoginResponse {
        message: "login_successful",
        user_email: user.email.to_string(),
    }))
}