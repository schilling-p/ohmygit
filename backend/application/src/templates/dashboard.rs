use axum::response::{IntoResponse, Html};
use axum_macros::debug_handler;
use templating::{DashboardTemplate};
use error::AppError;
use askama::Template;
use axum::extract::State;
use tower_sessions::Session;
use crate::repository::read::list_user_repositories;
use crate::organizations::read::list_user_organizations;

#[debug_handler]
pub async fn dashboard(session: Session, State(pool): State<deadpool_diesel::postgres::Pool> ) -> Result<impl IntoResponse, AppError> {
    let user_email: String = session.get("user_email").await?.unwrap_or("paul.schilling@code.berlin".to_string());
    let user_repositories = list_user_repositories(&pool, &user_email).await?;
    let user_organizations = list_user_organizations(&pool, &user_email).await?;
    let username: Option<String> = session.get("username").await?;

    let template = DashboardTemplate {
        name: username.unwrap_or("paul".to_string()),
        repositories: user_repositories,
        organizations: user_organizations,
    };
    let html = template.render()?;

    Ok(Html(html))
}