use axum::extract::{Path, State};
use axum::response::{Html, IntoResponse, Response};
use axum_macros::debug_handler;
use tower_sessions::Session;
use uuid::Uuid;
use templating::{DashboardTemplate, CreateRepositoryTemplate};
use error::AppError;
use state::AppState;

#[debug_handler]
pub async fn dashboard_template(session: Session, State(app_state): State<AppState> ) -> Result< impl IntoResponse, AppError> {
    let username: Option<String> = session.get("username").await?;
    let user_id: Option<Uuid> = session.get("user_id").await?;

    if let (Some(username), Some(user_id)) = (username, user_id) {
        let organizations = app_state.stores.members.list_organizations_for_user(user_id).await?;
        let repositories = app_state.stores.repos.list_user_repositories(user_id).await?;
        let template = DashboardTemplate {
            username,repositories, organizations,
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
pub async fn repository_creation_template(State(app_state): State<AppState>, session: Session) -> Result<Response, AppError> {
    let user_id: Option<Uuid> = session.get("user_id").await?;
    let username: Option<String> = session.get("username").await?;
    if let (Some(user_id), Some(username)) = (user_id, username) {
        let repos = app_state.stores.repos.list_user_repositories(user_id).await?;
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