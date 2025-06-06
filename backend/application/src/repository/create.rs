use axum::extract::State;
use axum::response::{IntoResponse, Redirect};
use axum_macros::debug_handler;
use axum::Json;
use tower_sessions::Session;
use domain::request::auth::UserIdentifier;
use crate::user::read::retrieve_user_from_db;
use domain::request::repository::CreateRepoRequest;
use error::AppError;
use infrastructure::diesel::DbPool;

#[debug_handler]
pub async fn create_repository(State(pool): State<DbPool>, session: Session, Json(create_repo_request): Json<CreateRepoRequest>) -> Result<impl IntoResponse, AppError> {
    // validate the repo name
    // ensure the repo does not already exist
    // build repo path
    // create a user folder in /repos if missing
    // init bare repo
    // insert repository metadata into the database
    // create a new struct for the database insertion
    
    let username: Option<String> = session.get("username").await?;
    if let Some(username) = username {
        let user = retrieve_user_from_db(&pool, UserIdentifier::Username(username)).await?;
        Ok(Redirect::to("/dashboard"))
        
    } else {
        Err(AppError::Unauthorized)
    }
    
}