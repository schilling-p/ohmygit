use std::io::Write;
use tokio::process::Command;
use std::path::PathBuf;
use std::process::Stdio;
use axum::extract::{Path, Query, State};
use headers::Authorization;
use axum::http::{header, StatusCode};
use axum_macros::debug_handler;
use axum::response::{Response};
use axum_extra::{headers, typed_header::TypedHeader};
use axum_extra::headers::authorization::Basic;
use diesel::{RunQueryDsl, SelectableHelper, QueryDsl, BelongingToDsl};
use diesel::expression_methods::ExpressionMethods;
use tracing::debug;
use chrono::DateTime;

use domain::models::{Repository, User};
use domain::request::auth::{UserIdentifier, LoginRequest};
use domain::request::repository::{InfoRefsQuery,AuthorizationRequest, RepoAction, Credentials};
use domain::response::repository::{RepositoryOverview, RepositoryFileInformation, CommitInformation};
use error::AppError;
use infrastructure::diesel::DbPool;
use crate::user::read::retrieve_user_from_db;
use crate::user::login::login_user;
use crate::repository::auth::authorize_repository_action;
use infrastructure::git2::GitRepository;

// test command: GIT_TRACE_PACKET=1 GIT_TRACE=1 git clone http://0.0.0.0:3001/paul/ohmygit.git
#[debug_handler]
pub async fn handle_info_refs(
    pool: State<DbPool>,
    Path((username, repo_name)): Path<(String, String)>,
    Query(query): Query<InfoRefsQuery>,
    opt_auth: Option<TypedHeader<Authorization<Basic>>>) -> Result<Response, AppError> {
    // find the repository,
    // figure out if the repository is public
    // if public, clone directly
    // if private, go through the authorization process
    // if the user is authorized to clone, good
    // if not, return 401

    let possible_operations = ["git-upload-pack".to_string(), "git-receive-pack".to_string()];
    if !possible_operations.contains(&query.service) {
        return Err(AppError::BadRequest(format!("Unsupported service: {:?}", query.service)))
    }

    // from here on out the query is supported and can be used
    let repo_name_no_git = repo_name.strip_suffix(".git").unwrap_or(&repo_name);
    let repo = find_repository_by_name(&pool, &repo_name_no_git).await?;
    debug!("found repository: {:?}", repo.name);
    let repo_path = PathBuf::from(format!("/repos/{}/{}.git", username, repo_name_no_git));

    if repo.is_public && query.service == "git-upload-pack" {
        let output = run_git_advertise_refs(&query.service, repo_path).await?;
        let formatted_output = format_git_advertisement(&query.service, &output);
        build_git_advertisement_response(&query.service, formatted_output)

    } else {
        let auth_header = opt_auth.ok_or(AppError::GitUnauthorized("Missing username or password from authorization header".into()))?;
        let action = RepoAction::try_from(query.service.as_str())?;

        authenticate_and_authorize_user(&pool ,auth_header, repo, action).await?;

        let output = run_git_advertise_refs(&query.service, repo_path).await?;
        let formatted_output = format_git_advertisement(&query.service, &output);
        build_git_advertisement_response(&query.service, formatted_output)
    }
}

#[debug_handler]
pub async fn receive_user_repository(pool: State<DbPool>, Path((username, repo_name)): Path<(String, String)>, opt_auth: Option<TypedHeader<Authorization<Basic>>>, body: axum::body::Bytes) -> Result<Response, AppError> {
    // enable users to push to the repository stored on disk in the backend container
    // get the repository as the body of the request
    // run the git-receive-pack command
    // send back the result of that command as the response body

    let repo_name_no_git = repo_name.strip_suffix(".git").unwrap_or(&repo_name);
    let repo = find_repository_by_name(&pool, repo_name_no_git).await?;
    let auth_header = opt_auth.ok_or(AppError::GitUnauthorized("Missing username or password from authorization header".into()))?;
    authenticate_and_authorize_user(&pool, auth_header, repo, RepoAction::Push).await?;

    let repo_path = PathBuf::from(format!("../repos/{}/{}.git", username, repo_name_no_git));
    debug!("receiving user repository: {:?}", &repo_path);

    let service: String = "git-receive-pack".to_string();
    let output = run_git_pack(&service, repo_path, body).await?;
    let response = build_git_pack_response(&service, output)?;
    Ok(response)
}

async fn authenticate_and_authorize_user(pool: &State<DbPool>, auth_header: TypedHeader<Authorization<Basic>>, repository: Repository, repo_action: RepoAction) -> Result<(), AppError> {
    let credentials = Credentials::from(auth_header);
    let login_request = LoginRequest {
        identifier: UserIdentifier::Username(credentials.username),
        password: credentials.password,
    };
    let user = login_user(&pool, login_request).await.map_err(|_| AppError::GitUnauthorized("Credentials don't check out.".to_string()))?;

    let auth_request = AuthorizationRequest {
        user, repo: repository, action: repo_action,
    };
    authorize_repository_action(&pool, auth_request).await?;

    Ok(())
}

#[debug_handler]
pub async fn send_user_repository(pool: State<DbPool>, Path((username, repo_name)): Path<(String, String)>, opt_auth: Option<TypedHeader<Authorization<Basic>>>, body: axum::body::Bytes) -> Result<Response, AppError> {
    let auth_header = opt_auth.ok_or(AppError::GitUnauthorized("Missing username or password from authorization header".into()))?;
    let repo_name_no_git = repo_name.strip_suffix(".git").unwrap_or(&repo_name);
    let repo_path = PathBuf::from(format!("../repos/{}/{}", username, repo_name_no_git));
    let repo = find_repository_by_name(&pool, repo_name_no_git).await?;
    authenticate_and_authorize_user(&pool, auth_header, repo, RepoAction::Clone).await?;

    debug!("sending user repository: {:?}", &repo_path);
    let service: &str = "git-upload-pack";
    let output = run_git_pack(&service, repo_path, body).await?;
    let response = build_git_pack_response(service, output)?;

    Ok(response)
}

async fn find_repository_by_name(pool: &DbPool, repo_name: &str) -> Result<Repository, AppError> {
    use domain::schema::repositories::dsl::*;
    let conn = pool.get().await.map_err(AppError::from)?;
    let repo_name_owned = repo_name.to_owned();
    let repo = conn
        .interact(move |conn| repositories.filter(name.eq(repo_name_owned)).select(Repository::as_select()).first::<Repository>(conn))
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .map_err(AppError::from)?;
    Ok(repo)
}

pub fn get_repo_overview(repo_path: &str) -> Result<RepositoryOverview, AppError> {
    let git_repo = GitRepository::open(repo_path)?;
    let repo_name = git_repo.get_repository_name()?;
    let head_commit = git_repo.get_head_commit()?;
    let tree = head_commit.tree()?;

    let mut files: Vec<RepositoryFileInformation> = Vec::new();
    for entry in tree.iter() {
        let file_name = entry.name().unwrap_or("").to_string();
        let (message, timestamp) = git_repo.get_last_commit_for_path(&file_name)?;
        files.push(RepositoryFileInformation {
            file_name,
            last_commit_message: message,
            last_commit_time: timestamp,
        });
    }

    let head_commit_time = DateTime::from_timestamp(head_commit.time().seconds(), 0).unwrap();
    let commit_information = CommitInformation {
        commit_message: head_commit.message().unwrap_or("no commit yet").to_string(),
        commit_time: head_commit_time.to_string(),
    };
    
    Ok(RepositoryOverview {
        repository_name: repo_name,
        latest_commit: commit_information,
        files,
    })
}

async fn run_git_advertise_refs(service: &str, repo_path: PathBuf) -> Result<Vec<u8>, AppError> {
    debug!("running git advertise refs for service: {:?}", service);
    let output = Command::new(service)
        .arg("--stateless-rpc")
        .arg("--advertise-refs")
        .arg(repo_path)
        .output()
        .await
        .map_err(|err| AppError::InternalServerError(format!("Git spawn error: {:?}", err)))?;

    if !output.status.success() {
        return Err(AppError::InternalServerError(format!("Git error: {:?}", output.status)));
    }

    Ok(output.stdout)
}

fn format_git_advertisement(service: &str, body: &[u8]) -> Vec<u8> {
    let mut formatted_output: Vec<u8> = Vec::new();

    let service_line = format!("# service={}\n", service);
    let pkt_line_len = service_line.len() + 4;
    write!(&mut formatted_output, "{:04x}", pkt_line_len).unwrap();
    formatted_output.extend_from_slice(service_line.as_bytes());
    formatted_output.extend_from_slice(b"0000");
    formatted_output.extend_from_slice(body);

    formatted_output
}

async fn run_git_pack(service: &str, repo_path: PathBuf, body: axum::body::Bytes) -> Result<Vec<u8>, AppError> {
    debug!("running git pack for service: {:?}", service);
    debug!("running git pack with repo path: {:?}", repo_path);
    let mut child = Command::new(service)
        .arg("--stateless-rpc")
        .arg(repo_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| AppError::InternalServerError(format!("Git spawn error: {:?}", err)))?;

    if let Some(mut stdin) = child.stdin.take() {
        use tokio::io::AsyncWriteExt;
        stdin.write_all(&body).await.map_err(|err| AppError::InternalServerError(format!("Git stdin error: {:?}", err)))?;
    }

    let mut stdout = child
        .stdout
        .take()
        .ok_or(AppError::InternalServerError("Failed to capture stdout".into()))?;

    use tokio::io::AsyncReadExt;
    let mut output = Vec::new();
    stdout.read_to_end(&mut output).await.map_err(|err| AppError::InternalServerError(format!("Git stdout error: {:?}", err)))?;

    let status = child.wait().await.map_err(|err| AppError::InternalServerError(format!("Git stdout error: {:?}", err)))?;
    if !status.success() {
        return Err(AppError::InternalServerError(format!("Git execution error: {:?}", status)));
    }

    Ok(output)
}

fn build_git_advertisement_response(service: &str, formatted_output: Vec<u8>) -> Result<Response, AppError> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, format!("application/x-{}-advertisement", service))
        .header(header::CACHE_CONTROL, "no-cache")
        .body(formatted_output.into())
        .unwrap())
}

fn build_git_pack_response(service: &str, output: Vec<u8>) -> Result<Response, AppError> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, format!("application/x-{}-result", service))
        .header(header::CACHE_CONTROL, "no-cache")
        .body(output.into())
        .unwrap()
    )
}

pub async fn list_user_repositories(pool: &DbPool, user_email: &str) -> Result<Vec<Repository>, AppError> {
    debug!("listing user repositories for: {:?}", user_email);
    let user: User = retrieve_user_from_db(&pool, UserIdentifier::Email((&user_email).parse::<String>().unwrap())).await?;
    let conn = pool.get().await.map_err(AppError::from)?;
    let repos = conn
        .interact(move |conn| Repository::belonging_to(&user).select(Repository::as_select()).load(conn))
        .await
        .map_err(|e| AppError::UnexpectedError(e.to_string()))?
        .map_err(AppError::from)?;

    Ok(repos)
}