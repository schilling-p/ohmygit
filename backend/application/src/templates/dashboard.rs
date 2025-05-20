use axum::response::{IntoResponse, Html};
use axum_macros::debug_handler;
use templating::{DashboardTemplate};
use error::AppError;
use askama::Template;
use axum::extract::{State};
use tower_sessions::Session;
use crate::repository::read::list_user_repositories;
use crate::organizations::read::list_user_organizations;
use convert_case::{Case, Casing};

#[debug_handler]
pub async fn dashboard(session: Session, State(pool): State<deadpool_diesel::postgres::Pool> ) -> Result<impl IntoResponse, AppError> {
    let user_email: String = session.get("user_email").await?.unwrap();
    let user_repositories = list_user_repositories(&pool, &user_email).await?;
    let user_organizations = list_user_organizations(&pool, &user_email).await?;
    let username: Option<String> = session.get("username").await?;

    let template = DashboardTemplate {
        username: username.unwrap_or("".to_string()).to_case(Case::Pascal),
        repositories: user_repositories,
        organizations: user_organizations,
    };
    let html = template.render()?;

    Ok(Html(html))
}