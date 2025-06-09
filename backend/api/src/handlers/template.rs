use axum::extract::{Path, State};
use axum::response::{Html, IntoResponse, Response};
use axum_macros::debug_handler;
use tower_sessions::Session;
use application::organizations::read::list_user_organizations;
use application::repository::read::list_user_repositories;
use domain::request::auth::UserIdentifier;
use error::AppError;
use state::AppState;

#[debug_handler]
pub async fn dashboard_template(session: Session, State(app_state): State<AppState> ) -> Result< impl IntoResponse, AppError> {
    
    let user_email: Option<String> = session.get("user_email").await?;
    let username: Option<String> = session.get("username").await?;

    if let (Some(user_email), Some(username)) = (user_email, username) {
        let user = app_state.stores.users.retrieve_user_by_identifier(UserIdentifier::Username(username)).await?;
        let template = DashboardTemplate {
            username: user.username,
            repositories: app_state.stores.repos.list_user_repositories(user.id).await?,
            organizations: list_user_organizations(&pool, &user_email).await?,
        };

        Ok(Html(template.render()?))

    } else {
        Err(AppError::Unauthorized)
    }
}

#[debug_handler]
pub async fn repository_template_default(State(app_state): State<AppState>, Path((username, repository_name)): Path<(String, String)>, session: Session) -> Result<Response, AppError> {
    let Some(current_user) = session.get::<String>("username").await? else {
        return Err(AppError::Unauthorized);
    };

    let pool = &app_state.db;
    let branch_name = None;
    create_repository_view(pool, current_user, repository_name, branch_name, session).await
}

#[debug_handler]
pub async fn repository_template_for_branch(State(app_state): State<AppState>, Path((username, repository_name, branch_name)): Path<(String, String, String)>, session: Session) -> Result<Response, AppError> {
    let Some(current_user) = session.get::<String>("username").await? else {
        return Err(AppError::Unauthorized);
    };
    let pool = &app_state.db;
    let branch_name = Some(branch_name);
    create_repository_view(pool, current_user, repository_name, branch_name, session).await
}

#[debug_handler]
pub async fn create_repository_template(State(app_state): State<AppState>, session: Session) -> Result<Response, AppError> {
    let pool = &app_state.db;
    let user_email: Option<String> = session.get("user_email").await?;
    let username: Option<String> = session.get("username").await?;
    if let (Some(user_email), Some(username)) = (user_email, username) {
        let repos: Vec<Repository> = list_user_repositories(&pool, &user_email).await?;
        let mut repo_names = Vec::new();
        for repo in repos.into_iter() {
            repo_names.push(repo.name);
        }
        let template = CreateRepositoryTemplate {
            username,
            repositories: repo_names,
        };

        Ok(Html(template.render()?).into_response())

    } else {
        Err(AppError::Unauthorized)
    }
}