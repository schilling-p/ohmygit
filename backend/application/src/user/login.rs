use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum_macros::debug_handler;
use axum::Json;
use axum::response::Response;
use super::read::retrieve_user_from_db;
use shared::crypto::verify_password;
use error::AppError;
use tracing::debug;
use tower_sessions::Session;

use domain::request::auth::{LoginRequest};
use domain::response::auth::LoginResponse;
use domain::ApiResponse;
use domain::user::{User, NewUser};
use super::service::UserService;

impl UserService {
    pub async fn user_login(&self, login_request: LoginRequest) -> Result<User, AppError> {
        let user = self.store.retrieve_user_by_identifier(login_request.identifier).await?;
        verify_password(&login_request.password, &user.hashed_pw)?;
        Ok(user)
    }
}