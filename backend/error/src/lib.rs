use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use diesel::result::Error as DieselError;
use serde_json::json;
use argon2::password_hash::Error as PasswordHashError;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    EmailAlreadyExists,
    DatabaseError(DieselError),
    UnexpectedError(String),
    PoolError(deadpool_diesel::PoolError),
    JoinError(tokio::task::JoinError),
    PasswordHashError(PasswordHashError),
}

impl From<PasswordHashError> for AppError {
    fn from(err: PasswordHashError) -> Self {
        AppError::PasswordHashError(err)
    }
}

impl From<DieselError> for AppError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => AppError::NotFound("Not Found".to_string()),
            other => AppError::DatabaseError(other),
        }
    }
}

impl From<tokio::task::JoinError> for AppError {
    fn from(err: tokio::task::JoinError) -> Self {
        AppError::JoinError(err)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::UnexpectedError(err.to_string())
    }
}

impl From<deadpool_diesel::PoolError> for AppError {
    fn from(err: deadpool_diesel::PoolError) -> Self {
        AppError::PoolError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound(msg) => {
                let body = Json(json!({"error": msg}));
                (StatusCode::NOT_FOUND, body).into_response()
            }
            AppError::DatabaseError(err) => {
                let body = Json(json!({"error": err.to_string()}));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
            AppError::UnexpectedError(msg) => {
                let body = Json(json!({"error": msg}));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
            AppError::PoolError(err) => {
                let body = Json(json!({"error": err.to_string()}));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
            AppError::JoinError(err) => {
                let body = Json(json!({"error": err.to_string()}));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
            AppError::PasswordHashError(err) => {
                let body = Json(json!({"error": "password_hash_error", "message": err.to_string()}));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
            AppError::EmailAlreadyExists => {
                let body = Json(json!({"error": "email_already_exists"}));
                (StatusCode::CONFLICT, body).into_response()
            }
        }
    }
}