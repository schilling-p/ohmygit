use axum::response::{IntoResponse, Html};
use templating::{RepositoryTemplate};
use error::AppError;
use askama::Template;
use axum::extract::{Path, State};
use axum_macros::debug_handler;
use tower_sessions::Session;
use convert_case::{Case, Casing};
use crate::repository::read::get_repo_overview;
use crate::repository::read::find_repository_by_name;
use crate::user::read::retrieve_user_from_db;
use crate::repository::auth::authorize_repository_action;
use domain::request::repository::{AuthorizationRequest, RepoAction};
use infrastructure::diesel::DbPool;
use tracing::debug;
use domain::request::auth::UserIdentifier;

#[debug_handler]
pub async fn repository_template(pool: State<DbPool>, Path((username, repository_name)): Path<(String, String)>, session: Session) -> Result<impl IntoResponse, AppError> {
    let repo_owner = username;
    let repo_name = repository_name;
    let Some(current_user) = session.get::<String>("username").await? else {
        return Err(AppError::Unauthorized);
    };
    
    let repository = find_repository_by_name(&pool, &repo_name).await?;
    if !repository.is_public {
        let repo_action = RepoAction::View;
        let user = retrieve_user_from_db(&pool, UserIdentifier::Username(current_user)).await?;
        let auth_request = AuthorizationRequest {
            user, repository, repo_action
        };
        authorize_repository_action(&pool, auth_request).await?;
    }    

    let repo_path = format!("/repos/{}/{}.git", repo_owner, repo_name);
    let repo_overview = get_repo_overview(&repo_path)?;
    let template = RepositoryTemplate {
        repository_name: repo_name,
        username: repo_owner.to_case(Case::Pascal),
        overview: repo_overview,
    };

    let html = template.render()?;

    Ok(Html(html))
}