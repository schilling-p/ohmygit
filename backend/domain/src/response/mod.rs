use serde::Serialize;
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use crate::response::repository::{RepositoryBranches, RepositoryOverview};
use super::response::auth::{SignupResponse, LoginResponse};
use super::response::health::HealthResponse;

pub mod health;
pub mod auth;
pub mod repository;

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(tag = "type", content = "data")]
pub enum ApiResponse {
    Login(LoginResponse),
    Signup(SignupResponse),
    Health(HealthResponse),
    RepositoryForUser(RepositoryOverview),
    RepositoryBranches(RepositoryBranches)
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        let status = match self {
            ApiResponse::Health(_) => {StatusCode::OK},
            ApiResponse::Login(_) => {StatusCode::OK},
            ApiResponse::Signup(_) => {StatusCode::CREATED},
            ApiResponse::RepositoryForUser(_) => {StatusCode::OK},
            ApiResponse::RepositoryBranches(_) => {StatusCode::OK}
        };

        (status, Json(self)).into_response()
    }
}