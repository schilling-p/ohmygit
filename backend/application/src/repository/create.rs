use axum::extract::State;
use axum::response::{IntoResponse, Redirect};
use axum_macros::debug_handler;
use axum::Json;
use tower_sessions::Session;
use regex::Regex;
use std::sync::LazyLock;
use std::path::PathBuf;
use tokio::fs;

use domain::request::auth::UserIdentifier;
use crate::user::read::retrieve_user_from_db;
use domain::request::repository::CreateRepoRequest;
use error::AppError;
use infrastructure::diesel::DbPool;
use infrastructure::git2::GitRepository;

#[debug_handler]
pub async fn create_repository(State(pool): State<DbPool>, session: Session, Json(create_repo_request): Json<CreateRepoRequest>) -> Result<impl IntoResponse, AppError> {
    // ensure the repo does not already exist - DONE
    // build repo path - DONE
    // create a user folder in /repos if missing - DONE
    // init the bare repository - DONE
    // create a new struct for the database insertion
    // insert repository metadata into the database

    if !is_valid_repo_name(&create_repo_request.repository_name) {
        return Err(AppError::BadRequest("Invalid name for a repository".to_string()))
    }

    let username: Option<String> = session.get("username").await?;
    if let Some(username) = username {
        let user = retrieve_user_from_db(&pool, UserIdentifier::Username(username)).await?;
        let repo_path = format!("/repos/{}/{}", &user.username, &create_repo_request.repository_name);
        match GitRepository::open(&repo_path) {
            Ok(_) => return Err(AppError::BadRequest("Repository already exists".to_string())),
            Err(_) => {},
        }

        let user_directory = PathBuf::from(format!("/repos/{}", &user.username));
        if !fs::try_exists(&user_directory).await? {
            fs::create_dir_all(&user_directory).await?;
        }
        
        GitRepository::init_bare(&repo_path)?;
        
        Ok(Redirect::to("/dashboard"))


    } else {
        Err(AppError::Unauthorized)
    }
}

static REPO_NAME_REGEX: LazyLock<Regex> = LazyLock::new( || {
    Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9_-]{0,31}$").unwrap()
});

fn is_valid_repo_name(repo_name: &str) -> bool {
    REPO_NAME_REGEX.is_match(repo_name)
}