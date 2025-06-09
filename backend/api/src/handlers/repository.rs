use std::path::PathBuf;
use axum::extract::{Path, State};
use axum::Json;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum_macros::debug_handler;
use tokio::fs;
use regex::Regex;
use std::sync::LazyLock;
use tower_sessions::Session;
use tracing::debug;
use application::repository::read::list_user_repositories;
use domain::ApiResponse;
use domain::request::auth::UserIdentifier;
use domain::request::repository::CreateRepoRequest;
use domain::response::repository::RepositoryBranches;
use state::AppState;
use error::AppError;
use infrastructure::git2::GitRepository;

#[debug_handler]
pub async fn list_repository_branches(Path((username, repo_name)): Path<(String, String)>) -> Result<ApiResponse, AppError> {
    // TODO: figure out why this does not check the session, does no auth at all
    let repo_path = format!("/repos/{}/{}.git", username, repo_name);
    let git_repo = GitRepository::open(&repo_path)?;
    let branches = RepositoryBranches {branches: git_repo.list_local_branches()?};
    Ok(ApiResponse::RepositoryBranches(branches))
}

#[debug_handler]
pub async fn create_repository(State(app_state): State<AppState>, session: Session, Json(create_repo_request): Json<CreateRepoRequest>) -> Result<impl IntoResponse, AppError> {
    // ensure the repo does not already exist - DONE
    // build repo path - DONE
    // create a user folder in /repos if missing - DONE
    // init the bare repository - DONE
    // create a new struct for the database insertion 
    // insert repository metadata into the database
    debug!("Got request with: {:?}", create_repo_request);

    if !is_valid_repo_name(&create_repo_request.repository_name) {
        return Err(AppError::BadRequest("Invalid name for a repository".to_string()))
    }

    let username: Option<String> = session.get("username").await?;
    if let Some(username) = username {
        let user = app_state.stores.users.retrieve_user_by_identifier(UserIdentifier::Username(username.clone())).await?;
        let repo_path = format!("/repos/{}/{}.git", &user.username, &create_repo_request.repository_name);
        
        match GitRepository::open(&repo_path) {
            Ok(_) => return Err(AppError::BadRequest("Repository already exists".to_string())),
            Err(_) => {},
        }

        let user_directory = PathBuf::from(format!("/repos/{}", &user.username));
        if !fs::try_exists(&user_directory).await? {
            fs::create_dir_all(&user_directory).await?;
        }

        GitRepository::init_bare(&repo_path)?;
        
        app_state.services.repo.create_new_user_repository(username, create_repo_request).await?;

        let redirect_url = format!("/repos/{}/{}", &user.username, &create_repo_request.repository_name);
        Ok(Redirect::to(&redirect_url))


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


