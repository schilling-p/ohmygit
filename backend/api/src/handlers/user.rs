use axum::extract::State;
use axum_macros::debug_handler;
use axum::Json;
use tower_sessions::Session;
use domain::ApiResponse;
use domain::request::auth::{LoginRequest};
use domain::response::auth::LoginResponse;
use domain::user::{NewUser, User};
use state::AppState;
use error::AppError;

#[debug_handler]
pub async fn list_users(State(app_state): State<AppState>) -> Result<Json<Vec<User>>, AppError> {
    let users = app_state.stores.users.list_users().await?;
    Ok(Json(users))
}

#[debug_handler]
pub async fn user_web_signup_handler(State(app_state): State<AppState>, Json(mut new_user): Json<NewUser>) -> Result<Json<User>, AppError> {
    match app_state.services.user.user_signup(new_user).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e),
    }
}

#[debug_handler]
pub async fn user_web_login_handler(session: Session, State(app_state): State<AppState>, Json(login_request): Json<LoginRequest>) -> Result<ApiResponse, AppError> {
    match app_state.services.user.user_login(login_request) {
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