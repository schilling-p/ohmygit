use axum::response::{IntoResponse, Html};
use templating::{RepositoryTemplate};
use error::AppError;
use askama::Template;
use axum::extract::{Path};
use axum_macros::debug_handler;
use tower_sessions::Session;
use crate::repository::read::get_repo_overview;
use tracing::debug;

#[debug_handler]
pub async fn repository(Path(repository_name): Path<String>, session: Session) -> Result<impl IntoResponse, AppError> {
    let username: String = session.get("username").await?.ok_or(AppError::Unauthorized)?;
    debug!("username: {:?}", &username);
    let repo_path = format!("/repos/{}/{}.git", username, repository_name);
    debug!("repo_path: {:?}", &repo_path);
    let repo_overview = get_repo_overview(&repo_path)?;
    let template = RepositoryTemplate {
        username: username.to_uppercase(),
        overview: repo_overview,
    };

    let html = template.render()?;

    Ok(Html(html))
}