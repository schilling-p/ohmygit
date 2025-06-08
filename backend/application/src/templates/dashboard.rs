use axum::response::{IntoResponse, Html};
use axum_macros::debug_handler;
use templating::{DashboardTemplate};
use error::AppError;
use askama::Template;
use axum::extract::{State};
use tower_sessions::Session;
use shared::state::AppState;
use crate::repository::read::list_user_repositories;
use crate::organizations::read::list_user_organizations;

#[debug_handler]
pub async fn dashboard_template(session: Session, State(app_state): State<AppState> ) -> Result< impl IntoResponse, AppError> {
    let pool = &app_state.db;
    let user_email: Option<String> = session.get("user_email").await?;
    let username: Option<String> = session.get("username").await?;
    
    if let (Some(user_email), Some(username)) = (user_email, username) {
        let template = DashboardTemplate {
            username,
            repositories: list_user_repositories(&pool, &user_email).await?,
            organizations: list_user_organizations(&pool, &user_email).await?,
        };

        Ok(Html(template.render()?))

    } else {
        Err(AppError::Unauthorized)
    }
}