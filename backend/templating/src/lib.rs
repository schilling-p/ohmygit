use askama::Template;
use axum::response::{IntoResponse, Response, Html};
use axum::http::StatusCode;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub name: String,
}