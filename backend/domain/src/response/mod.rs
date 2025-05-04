use serde::Serialize;
use serde_json::json;
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};

use super::response::auth::{SignupResponse, LoginResponse};
use super::response::health::HealthResponse;

pub mod health;
pub mod auth;

#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ApiResponse {
    Login(LoginResponse),
    Signup(SignupResponse),
    Health(HealthResponse),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        let status = match self {
            ApiResponse::Health(_) => {StatusCode::OK},
            ApiResponse::Login(_) => {StatusCode::OK},
            ApiResponse::Signup(_) => {StatusCode::CREATED},
        };

        (status, Json(self)).into_response()
    }
}