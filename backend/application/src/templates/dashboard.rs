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
pub async fn dashboard_template(session: Session, State(pool): State<deadpool_diesel::postgres::Pool> ) -> Result< impl IntoResponse, AppError> {
    let user_email: Option<String> = session.get("user_email").await?;
    let username: Option<String> = session.get("username").await?;

    if let (Some(user_email), Some(username)) = (user_email, username) {
        let template = DashboardTemplate {
            username: username.to_case(Case::Pascal),
            repositories: list_user_repositories(&pool, &user_email).await?,
            organizations: list_user_organizations(&pool, &user_email).await?,
        };

        Ok(Html(template.render()?))

    } else {
        Err(AppError::Unauthorized)
    }
}