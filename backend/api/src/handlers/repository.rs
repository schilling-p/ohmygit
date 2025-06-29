use std::path::PathBuf;
use axum::extract::{Path, Query, State};
use axum::Json;
use axum::response::{IntoResponse, Response};
use axum_macros::debug_handler;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Basic;
use axum_extra::TypedHeader;
use http::StatusCode;
use tower_sessions::Session;
use tracing::debug;

use application::repository::git::{GIT_CLONE_PACK, GIT_PUSH_PACK, GIT_REPO_PATH};
use application::repository::git::format::{build_git_advertisement_response, build_git_pack_response, format_git_advertisement};
use application::repository::git::run::{run_git_advertise_refs, run_git_pack};
use domain::ApiResponse;
use domain::authorization::model::{Credentials, RepoAction, AuthorizationRequest};
use domain::repository::model::NewRepositoryBranch;
use domain::request::auth::UserIdentifier;
use domain::request::repository::{CreateBranchRequest, CreateRepoRequest, InfoRefsQuery};
use domain::response::repository::RepositoryBranches;
use state::AppState;
use error::AppError;
use infrastructure::git2::GitRepository;
use shared::regex::is_valid_repo_name;

#[debug_handler]
pub async fn list_repository_branches(State(app_state): State<AppState>, Path((username, repo_name)): Path<(String, String)>) -> Result<ApiResponse, AppError> {
    // TODO: figure out why this does not check the session, does no auth at all
    let repo_path = format!("/repos/{}/{}.git", username, repo_name);
    let branches = RepositoryBranches {branches: app_state.stores.git_repos.as_ref().list_local_branches(&repo_path).await?};

    Ok(ApiResponse::RepositoryBranches(branches))
}

#[debug_handler]
pub async fn create_repository(State(app_state): State<AppState>, session: Session, Json(create_repo_request): Json<CreateRepoRequest>) -> Result<impl IntoResponse, AppError> {
    debug!("Got request with: {:?}", create_repo_request);

    if !is_valid_repo_name(&create_repo_request.repository_name) {
        return Err(AppError::BadRequest("Invalid name for a repository".to_string()))
    }

    let username: Option<String> = session.get("username").await?;
    if let Some(username) = username {
        debug!("current username is: {}", username);
        let repo_path = format!("/repos/{}/{}.git", &username, &create_repo_request.repository_name);
        debug!("now opening repo at path: {}", &repo_path);

        match GitRepository::open(&repo_path) {
            Ok(_) => return Err(AppError::BadRequest("Repository already exists".to_string())),
            Err(_) => {},
        }

        app_state.services.repo.create_new_user_repository(username, create_repo_request).await?;

        Ok(StatusCode::OK.into_response())

    } else {
        Err(AppError::Unauthorized)
    }
}

// test command: GIT_TRACE_PACKET=1 GIT_TRACE=1 git clone http://0.0.0.0:3001/paul/ohmygit.git
#[debug_handler]
pub async fn handle_info_refs(
    State(app_state): State<AppState>,
    Path((username, repo_name)): Path<(String, String)>,
    Query(query): Query<InfoRefsQuery>,
    opt_auth: Option<TypedHeader<Authorization<Basic>>>) -> Result<Response, AppError> {
    let possible_operations = [GIT_CLONE_PACK.to_string(), GIT_PUSH_PACK.to_string()];
    if !possible_operations.contains(&query.service) {
        return Err(AppError::BadRequest(format!("Unsupported service: {:?}", query.service)))
    }

    let repo_name_no_git = repo_name.strip_suffix(".git").unwrap_or(&repo_name); // use that for the query retrieve_by_name
    let repo = app_state.stores.repos.retrieve_by_name(&repo_name_no_git).await?;
    let repo_path = PathBuf::from(format!("{GIT_REPO_PATH}/{}/{}.git", username, repo_name_no_git));

    if repo.is_public && (query.service == GIT_CLONE_PACK) {
        let output = run_git_advertise_refs(&query.service, repo_path).await?;
        let formatted_output = format_git_advertisement(&query.service, &output);
        build_git_advertisement_response(&query.service, formatted_output)

    } else {
        let auth_header = opt_auth.ok_or(AppError::GitUnauthorized("Missing username or password from authorization header".into()))?;
        let credentials = Credentials::from(auth_header);
        let action = RepoAction::try_from(query.service.as_str())?;
        app_state.services.auth.authenticate_and_authorize_user(credentials, repo, action).await?;


        let output = run_git_advertise_refs(&query.service, repo_path).await?;
        let formatted_output = format_git_advertisement(&query.service, &output);
        build_git_advertisement_response(&query.service, formatted_output)
    }
}
#[debug_handler]
pub async fn send_user_repository(State(app_state): State<AppState>, Path((username, repo_name)): Path<(String, String)>, opt_auth: Option<TypedHeader<Authorization<Basic>>>, body: axum::body::Bytes) -> Result<Response, AppError> {
    let auth_header = opt_auth.ok_or(AppError::GitUnauthorized("Missing username or password from authorization header".into()))?;
    let credentials = Credentials::from(auth_header);
    let repo_name_no_git = repo_name.strip_suffix(".git").unwrap_or(&repo_name);
    let repo_path = PathBuf::from(format!("{GIT_REPO_PATH}/{}/{}", username, repo_name_no_git));
    let repo = app_state.stores.repos.retrieve_by_name(&repo_name_no_git).await?;
    app_state.services.auth.authenticate_and_authorize_user(credentials, repo, RepoAction::Clone).await?;

    debug!("sending user repository: {:?}", &repo_path);
    let service: &str = GIT_CLONE_PACK;
    let output = run_git_pack(&service, repo_path, body).await?;
    let response = build_git_pack_response(service, output)?;

    Ok(response)
}

#[debug_handler]
pub async fn receive_user_repository(State(app_state): State<AppState>, Path((username, repo_name)): Path<(String, String)>, opt_auth: Option<TypedHeader<Authorization<Basic>>>, body: axum::body::Bytes) -> Result<Response, AppError> {
    let repo_name_no_git = repo_name.strip_suffix(".git").unwrap_or(&repo_name);
    let repo = app_state.stores.repos.retrieve_by_name(&repo_name_no_git).await?;
    let auth_header = opt_auth.ok_or(AppError::GitUnauthorized("Missing username or password from authorization header".into()))?;
    let credentials = Credentials::from(auth_header);
    app_state.services.auth.authenticate_and_authorize_user(credentials, repo, RepoAction::Push).await?;

    let repo_path = PathBuf::from(format!("{GIT_REPO_PATH}/{}/{}.git", username, repo_name_no_git));

    let service: String = GIT_PUSH_PACK.to_string();
    let output = run_git_pack(&service, repo_path, body).await?;
    let response = build_git_pack_response(&service, output)?;
    Ok(response)
}

#[debug_handler]
pub async fn create_repository_branch(State(app_state): State<AppState>, session: Session,  Path((username, repo_name)): Path<(String, String)>, Json(create_branch_request): Json<CreateBranchRequest>) -> Result<impl IntoResponse, AppError> {
    let Some(current_user) = session.get::<String>("username").await? else {
        return Err(AppError::Unauthorized);
    };

    let user = app_state.stores.users.retrieve_user_by_identifier(UserIdentifier::Username(current_user.clone())).await?;
    let repository = app_state.stores.repos.retrieve_by_name(&repo_name).await?;
    let repo_action = RepoAction::CreateBranch;
    let auth_request = AuthorizationRequest {
        user_id: user.id.clone(),
        owner_id: repository.owner_id.unwrap(),
        repository_id: repository.id,
        repo_action,
    };

    app_state.services.auth.authorize_repository_action(auth_request).await?;

    let repo_path = format!("/repos/{}/{}.git", &username, &repo_name);
    //let git_repo = GitRepository::open(&repo_path)?;
    app_state.stores.git_repos.as_ref().create_branch(&repo_path, &create_branch_request.new_branch_name, &create_branch_request.base_branch_name, create_branch_request.switch_head).await?;

    let new_repo_branch = NewRepositoryBranch {
        creator_id: user.id,
        repository_id: repository.id,
        name: create_branch_request.new_branch_name.clone(),
    };
    app_state.stores.repos.write_repo_branch_to_db(new_repo_branch).await?;


    let recently_authorized_key = format!("{}:{}", &username, &repo_name);
    session.insert("recently_authorized_repo", recently_authorized_key).await?;
    debug!("Session has been updated");

    let redirect_url = format!("/repos/{}/{}/branch/{}", username, repo_name, create_branch_request.new_branch_name);
    debug!("Redirecting to: {}", redirect_url);
    Ok(StatusCode::OK)
}