use axum::response::{IntoResponse, Html, Response};
use templating::{RepositoryTemplate, CreateRepositoryTemplate};
use error::AppError;
use askama::Template;
use axum::extract::{Path, State};
use axum_macros::debug_handler;
use tower_sessions::Session;
use crate::repository::read::get_repo_overview;
use crate::repository::read::{find_repository_by_name, list_user_repositories};
use crate::user::read::retrieve_user_from_db;
use crate::repository::auth::authorize_repository_action;
use domain::request::repository::{AuthorizationRequest, RepoAction};
use infrastructure::diesel::DbPool;
use tracing::debug;
use domain::models::Repository;
use domain::request::auth::UserIdentifier;
use shared::state::AppState;

#[debug_handler]
pub async fn repository_template_default(State(app_state): State<AppState>, Path((username, repository_name)): Path<(String, String)>, session: Session) -> Result<Response, AppError> {
    let pool = &app_state.db;
    let branch_name = None;
    create_repository_view(pool, username, repository_name, branch_name, session).await
}

#[debug_handler]
pub async fn repository_template_for_branch(State(app_state): State<AppState>, Path((username, repository_name, branch_name)): Path<(String, String, String)>, session: Session) -> Result<Response, AppError> {
    let pool = &app_state.db;
    let branch_name = Some(branch_name);
    create_repository_view(pool, username, repository_name, branch_name, session).await
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

async fn create_repository_view(pool: &DbPool, username: String, repository_name: String, branch_name: Option<String>, session: Session) -> Result<Response, AppError> {
    let Some(current_user) = session.get::<String>("username").await? else {
        return Err(AppError::Unauthorized);
    };
    
    let is_recently_authorized: bool = session
        .get::<String>("recently_authorized_repo")
        .await?
        .as_deref()
       == Some(&format!("{}:{}", username, repository_name));
    debug!("is_recently_authorized: {:?}", is_recently_authorized);

    let repository = find_repository_by_name(&pool, &repository_name).await?;
    debug!("repository: {:?}", repository);
    if !repository.is_public && !is_recently_authorized {
        let repo_action = RepoAction::View;
        let user = retrieve_user_from_db(&pool, UserIdentifier::Username(current_user)).await?;
        let auth_request = AuthorizationRequest {
            user, repository, repo_action
        };
        authorize_repository_action(&pool, auth_request).await?;
    }
    
    session.remove::<String>("recently_authorized_repo").await?;

    let repo_path = format!("/repos/{}/{}.git", username, repository_name);
    let repo_overview = get_repo_overview(&repo_path, branch_name.as_deref())?;
    let template = RepositoryTemplate {
        repository_name,
        username,
        overview: repo_overview,
    };

    Ok(Html(template.render()?).into_response())
}