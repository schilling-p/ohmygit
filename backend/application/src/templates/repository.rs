use axum::response::{IntoResponse, Html};
use templating::{RepositoryTemplate};
use error::AppError;
use askama::Template;
use axum::extract::{Path, State};
use axum_macros::debug_handler;
use tower_sessions::Session;
use crate::repository::read::get_repo_overview;
use crate::repository::read::find_repository_by_name;
use crate::user::read::retrieve_user_from_db;
use crate::repository::auth::authorize_repository_action;
use domain::request::repository::{AuthorizationRequest, RepoAction};
use infrastructure::diesel::DbPool;
use tracing::debug;
use domain::request::auth::UserIdentifier;

#[debug_handler]
pub async fn repository_template_default(State(pool): State<DbPool>, Path((username, repository_name)): Path<(String, String)>, session: Session) -> Result<impl IntoResponse, AppError> {
    let branch_name = None;
    create_repository_view(pool, username, repository_name, branch_name, session).await
}

#[debug_handler]
pub async fn repository_template_for_branch(State(pool): State<DbPool>, Path((username, repository_name, branch_name)): Path<(String, String, String)>, session: Session) -> Result<impl IntoResponse, AppError> {
    let branch_name = Some(branch_name);
    
    create_repository_view(pool, username, repository_name, branch_name, session).await
}

async fn create_repository_view(pool: DbPool, username: String, repository_name: String, branch_name: Option<String>, session: Session) -> Result<impl IntoResponse, AppError> {
    let Some(current_user) = session.get::<String>("username").await? else {
        return Err(AppError::Unauthorized);
    };

    let is_recently_authorized: bool = session
        .get::<String>("recently_authorized_repo")
        .await?
        .as_deref()
        == Some(&format!("{}:{}", username, repository_name));

    let repository = find_repository_by_name(&pool, &repository_name).await?;
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

    Ok(Html(template.render()?))
}