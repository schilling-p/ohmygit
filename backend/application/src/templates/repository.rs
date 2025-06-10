use axum::response::{IntoResponse, Html, Response};
use templating::{RepositoryTemplate, CreateRepositoryTemplate};
use error::AppError;
use askama::Template;
use axum::extract::{Path, State};
use axum_macros::debug_handler;
use tower_sessions::Session;
use crate::repository::read::get_repo_overview;
use crate::repository::read::{find_repository_by_name};
use crate::user::read::retrieve_user_from_db;
use crate::repository::auth::authorize_repository_action;
use domain::request::repository::{AuthorizationRequest, RepoAction};
use infrastructure::diesel::DbPool;
use tracing::debug;
use domain::models::Repository;
use domain::request::auth::UserIdentifier;
use state::AppState;



async fn create_repository_view(pool: &DbPool, username: String, repository_name: String, branch_name: Option<String>, session: Session) -> Result<Response, AppError> {
   
    
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