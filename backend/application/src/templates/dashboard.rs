use axum::response::{IntoResponse, Html};
use axum_macros::debug_handler;
use templating::{DashboardTemplate};
use error::AppError;
use askama::Template;

#[debug_handler]
pub async fn dashboard() -> Result<impl IntoResponse, AppError> {
    let template = DashboardTemplate {name: "paul".to_string()};
    let html = template.render()?;
    Ok(Html(html))
}