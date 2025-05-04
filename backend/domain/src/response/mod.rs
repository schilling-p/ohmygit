use serde::Serialize;
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use crate::response::organization::ListOrganizationsResponse;
use crate::response::repository::ListRepositoriesResponse;
use super::response::auth::{SignupResponse, LoginResponse};
use super::response::health::HealthResponse;

pub mod health;
pub mod auth;
pub mod repository;
pub mod organization;

#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ApiResponse {
    Login(LoginResponse),
    Signup(SignupResponse),
    Health(HealthResponse),
    Repositories(ListRepositoriesResponse),
    Organizations(ListOrganizationsResponse)
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        let status = match self {
            ApiResponse::Health(_) => {StatusCode::OK},
            ApiResponse::Login(_) => {StatusCode::OK},
            ApiResponse::Signup(_) => {StatusCode::CREATED},
            ApiResponse::Repositories(_) => {StatusCode::OK},
            ApiResponse::Organizations(_) => {StatusCode::OK},
        };

        (status, Json(self)).into_response()
    }
}